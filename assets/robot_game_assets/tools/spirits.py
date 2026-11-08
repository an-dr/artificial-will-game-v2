"""Turn regular sprite sheets into spirit versions.

Per frame (at native 48x48 resolution, then x2):
  * colours -> 4-step ghostly cyan/mint ramp by luminance, outline -> deep teal
  * translucent, fading out toward the bottom (no feet on the ground)
  * hovers 2px with a gentle bob, soft glow halo, drifting particles
Adds appear_down / vanish_down rows.
Usage: python3 spirits.py <outdir> <sheet.png> [...]
"""
import json
import math
import os
import random
import sys
from PIL import Image, ImageDraw

OUT = sys.argv[1]
SHEETS = sys.argv[2:]
SC, CELL = 2, 96
N = CELL // SC

RAMP = [(38, 110, 150), (64, 175, 200), (135, 228, 222), (225, 255, 245)]
OUTL = (16, 48, 78)
GLOW = (120, 255, 230)
SPARK = [(225, 255, 245), (150, 240, 230)]
BG = (34, 38, 56)


def lum(p):
    return (0.299 * p[0] + 0.587 * p[1] + 0.114 * p[2]) / 255


def spiritify(frame, t, nframes, seed, hover=2, alpha_mul=1.0, particles=True,
              converge=None):
    src = frame.resize((N, N), Image.NEAREST)
    px = src.load()
    ys = [y for y in range(N) for x in range(N) if px[x, y][3]]
    if not ys:
        return frame
    top, bot = min(ys), max(ys)
    h = max(1, bot - top)
    bob = -hover - (1 if math.sin(2 * math.pi * t / max(nframes, 1)) > 0.3 else 0)
    out = Image.new("RGBA", (N, N), (0, 0, 0, 0))
    o = out.load()
    solid = set()
    allpts = []
    for y in range(N):
        for x in range(N):
            p = px[x, y]
            if not p[3]:
                continue
            ny = y + bob
            if not 0 <= ny < N:
                continue
            l = lum(p)
            if p[:3] == (11, 11, 15):
                col = OUTL
            else:
                col = RAMP[0 if l < 0.25 else 1 if l < 0.45 else 2 if l < 0.68 else 3]
            frac = (y - top) / h
            a = 225 if frac < 0.6 else int(225 - (frac - 0.6) / 0.4 * 175)
            a = int(a * alpha_mul)
            allpts.append((x, ny))
            if a > 0:
                o[x, ny] = col + (a,)
                solid.add((x, ny))
    # glow halo (2 rings)
    for ring, ga in ((1, 80), (2, 30)):
        for x, y in list(solid):
            for dx in range(-ring, ring + 1):
                for dy in range(-ring, ring + 1):
                    if abs(dx) + abs(dy) != ring:
                        continue
                    q = (x + dx, y + dy)
                    if 0 <= q[0] < N and 0 <= q[1] < N and q not in solid and o[q][3] == 0:
                        o[q] = GLOW + (int(ga * alpha_mul),)
    # particles drifting upward
    if particles and solid:
        rnd = random.Random(seed)
        xs = [x for x, _ in solid]
        for k in range(4):
            bx = rnd.randint(min(xs) - 2, max(xs) + 2)
            by = bot + bob - rnd.randint(0, h)
            y = by - ((t * 2 + k * 3) % (h + 4))
            if 0 <= bx < N and 0 <= y < N:
                o[bx, y] = SPARK[k % 2] + (int(200 * alpha_mul),)
    if converge and allpts:  # ring of motes for appear/vanish
        cx = sum(x for x, _ in allpts) // len(allpts)
        cy = sum(y for _, y in allpts) // len(allpts)
        r = converge
        for k in range(10):
            ang = 2 * math.pi * k / 10 + r * 0.3
            x, y = round(cx + math.cos(ang) * r), round(cy + math.sin(ang) * r * 0.8)
            if 0 <= x < N and 0 <= y < N:
                o[x, y] = SPARK[k % 2] + (230,)
    return out.resize((CELL, CELL), Image.NEAREST)


def appear_frames(base, seed):
    out = []
    for k in range(6):
        out.append(spiritify(base, k, 6, seed, alpha_mul=k / 5 if k else 0.0,
                             particles=k > 2, converge=max(0, 14 - k * 3)))
    out[0] = spiritify(base, 0, 6, seed, alpha_mul=0.0, particles=False, converge=16)
    return out


os.makedirs(f"{OUT}/gifs/single", exist_ok=True)
for path in SHEETS:
    meta = json.load(open(path.replace(".png", ".json")))
    sheet = Image.open(path).convert("RGBA")
    base = os.path.basename(path).replace(".png", "")
    name = base.split("_", 1)[1]
    seed = sum(map(ord, name))
    rows = []
    for an, m in meta["animations"].items():
        frames = [sheet.crop((x, y, x + w, y + h)) for x, y, w, h in m["rects"]]
        sp = [spiritify(f, i, len(frames), seed + i) for i, f in enumerate(frames)]
        rows.append((an, sp, m["fps"], m["loop"]))
    idle0 = sheet.crop(tuple(meta["animations"]["idle_down"]["rects"][0][:2]) +
                       (meta["animations"]["idle_down"]["rects"][0][0] + CELL,
                        meta["animations"]["idle_down"]["rects"][0][1] + CELL))
    ap = appear_frames(idle0, seed)
    rows.append(("appear_down", ap, 10, False))
    rows.append(("vanish_down", list(reversed(ap)), 10, False))

    cols = max(len(r[1]) for r in rows)
    out = Image.new("RGBA", (cols * CELL, len(rows) * CELL), (0, 0, 0, 0))
    om = {"image": f"spirit_{name}.png", "frame_size": [CELL, CELL],
          "origin_feet": [48, 78], "animations": {}}
    for r, (an, frames, fps, lp) in enumerate(rows):
        for c, im in enumerate(frames):
            out.paste(im, (c * CELL, r * CELL))
        om["animations"][an] = {"row": r, "frames": len(frames), "fps": fps, "loop": lp,
                                "rects": [[c * CELL, r * CELL, CELL, CELL] for c in range(len(frames))]}
    out.save(f"{OUT}/spirit_{name}.png")
    json.dump(om, open(f"{OUT}/spirit_{name}.json", "w"), indent=1)

    def onbg(im, s):
        b = Image.new("RGBA", im.size, BG + (255,)); b.alpha_composite(im)
        return b.resize((CELL * s, CELL * s), Image.NEAREST).convert("RGB")
    for an, frames, fps, lp in rows:
        imgs = [onbg(f, 3) for f in frames]
        if not lp:
            imgs += [imgs[-1]] * 5
        imgs[0].save(f"{OUT}/gifs/single/{name}_{an}.gif", save_all=True,
                     append_images=imgs[1:], duration=1000 // fps, loop=0)
    gc, cs = 4, CELL * 2
    gr = (len(rows) + gc - 1) // gc
    gif = []
    for t in range(36):
        cv = Image.new("RGB", (gc * cs, gr * (cs + 14)), BG); d = ImageDraw.Draw(cv)
        for i, (an, frames, fps, lp) in enumerate(rows):
            k = t * fps // 12
            idx = k % len(frames) if lp else min(k % (len(frames) + 4), len(frames) - 1)
            x, y = (i % gc) * cs, (i // gc) * (cs + 14)
            cv.paste(onbg(frames[idx], 2), (x, y))
            d.text((x + 6, y + cs), an, fill=(200, 235, 235))
        gif.append(cv)
    gif[0].save(f"{OUT}/gifs/{name}_all.gif", save_all=True, append_images=gif[1:],
                duration=1000 // 12, loop=0)
    print(name, len(rows), "rows")
