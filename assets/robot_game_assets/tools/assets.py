"""Biome asset packs: tiles, corner-Wang transitions, animated tiles, prop atlases,
demo maps. Everything native-res then x2 (same as the character sheets).

Conventions
  * native tile 32x32  -> 64x64 px in the delivered PNGs (robot = 1 tile)
  * Wang (corner) sets: 16 tiles, index = NW*8 + NE*4 + SW*2 + SE*1,
    bit = 1 where the corner is the *upper* terrain (named first)
  * props: anchor = ground contact point (sort sprites by anchor y)
Usage: python3 assets.py <outdir>
"""
import json
import math
import os
import random
import sys
from PIL import Image, ImageDraw

OUT = sys.argv[1] if len(sys.argv) > 1 else "assets"
sys.argv = [sys.argv[0], "."]
import scenes  # noqa: E402
import biomes  # noqa: E402
import replicant_biomes  # noqa: E402

T = 32          # native tile
SC = 2          # export scale
BG = (40, 44, 58)

# --------------------------------------------------------------- palettes
GR = scenes.GR; CON = scenes.CON; DIRT = scenes.DIRT; IVY = scenes.IVY; FLOW = scenes.FLOW
SAND = biomes.SAND; WAT = biomes.WAT; METAL = biomes.METAL; PAVE = biomes.PAVE
RB = replicant_biomes
ROCK = RB.ROCK; PLANK = RB.PLANK; WHITE = RB.WHITE; SEA = RB.SEA; HSAND = RB.SAND


def up(im):
    return im.resize((im.width * SC, im.height * SC), Image.NEAREST)


class Tex:
    """Draw helpers on a single tile with toroidal wrap (so every tile self-tiles)."""
    def __init__(self, seed):
        self.im = Image.new("RGBA", (T, T), (0, 0, 0, 0))
        self.r = random.Random(seed)

    def px(self, x, y, c, a=255):
        x, y = int(x) % T, int(y) % T
        if a >= 255:
            self.im.putpixel((x, y), tuple(c[:3]) + (255,))
        else:
            b = self.im.getpixel((x, y)); t = a / 255
            self.im.putpixel((x, y), tuple(int(b[i] * (1 - t) + c[i] * t) for i in range(3)) + (255,))

    def rect(self, x0, y0, x1, y1, c, a=255):
        for y in range(int(y0), int(y1) + 1):
            for x in range(int(x0), int(x1) + 1):
                self.px(x, y, c, a)


# ------------------------------------------------------------ base textures
def t_grass(seed, flowers=0.012):
    t = Tex(seed); t.rect(0, 0, T - 1, T - 1, GR[0])
    for _ in range(70):
        x, y = t.r.randint(0, T - 1), t.r.randint(0, T - 1)
        c = t.r.choice([GR[1], GR[1], GR[2], GR[3]])
        t.px(x, y, c); t.px(x - 1, y + 1, c); t.px(x + 1, y + 1, c)
    for _ in range(int(T * T * flowers)):
        x, y = t.r.randint(0, T - 1), t.r.randint(0, T - 1)
        t.px(x, y, t.r.choice(FLOW)); t.px(x, y + 1, GR[2])
    return t.im


def t_dirt(seed):
    t = Tex(seed); t.rect(0, 0, T - 1, T - 1, DIRT[0])
    for _ in range(60):
        t.px(t.r.randint(0, T), t.r.randint(0, T), t.r.choice(DIRT[1:]))
    for _ in range(3):
        x, y = t.r.randint(0, T), t.r.randint(0, T)
        t.rect(x, y, x + 1, y, (120, 110, 100))
    return t.im


def t_slab(seed, moss=True):
    t = Tex(seed); r = t.r
    for sy in (0, 16):
        for sx in (0, 16):
            base = r.choice(CON[:3])
            t.rect(sx, sy, sx + 15, sy + 15, base)
            t.rect(sx, sy, sx + 15, sy, tuple(min(255, c + 14) for c in base))
            t.rect(sx, sy + 15, sx + 15, sy + 15, CON[3]); t.rect(sx + 15, sy, sx + 15, sy + 15, CON[3])
            for _ in range(r.randint(0, 2)):
                x, y = sx + r.randint(2, 13), sy + r.randint(2, 13)
                for _ in range(r.randint(3, 8)):
                    if sx <= x < sx + 15 and sy <= y < sy + 15:
                        t.px(x, y, (88, 90, 88))
                        if moss and r.random() < .3:
                            t.px(x + 1, y, IVY[1])
                    x += r.choice((-1, 0, 1)); y += r.choice((0, 1))
            if moss:
                for _ in range(r.randint(0, 4)):
                    t.px(sx + r.randint(0, 15), sy + 15, IVY[r.randint(0, 2)])
    return t.im


def t_sand(seed, pal=SAND):
    t = Tex(seed); t.rect(0, 0, T - 1, T - 1, pal[0])
    for _ in range(14):
        x, y = t.r.randint(0, T), t.r.randint(0, T)
        t.rect(x, y, x + t.r.randint(2, 4), y, pal[1])
    for _ in range(8):
        t.px(t.r.randint(0, T), t.r.randint(0, T), pal[-1] if len(pal) > 3 else pal[2])
    return t.im


def t_water(seed, frame=0, pal=(WAT[1], WAT[0], WAT[2])):
    t = Tex(seed); t.rect(0, 0, T - 1, T - 1, pal[0])
    r = random.Random(seed)
    for k in range(16):
        x, y = r.randint(0, T), r.randint(0, T)
        ln = r.randint(1, 4)
        dx = int(2 * math.sin((frame + k) * math.pi / 2))
        t.rect(x + dx, y, x + dx + ln, y, pal[1] if k % 3 else pal[2])
    return t.im


def t_metal(seed):
    t = Tex(seed); r = t.r
    for sy in (0, 16):
        for sx in (0, 16):
            base = r.choice(METAL[:3])
            t.rect(sx, sy, sx + 15, sy + 15, base)
            t.rect(sx, sy + 15, sx + 15, sy + 15, METAL[3]); t.rect(sx + 15, sy, sx + 15, sy + 15, METAL[3])
            for cx, cy in ((2, 2), (13, 2), (2, 13), (13, 13)):
                t.px(sx + cx, sy + cy, METAL[2] if base != METAL[2] else METAL[0])
            for k in range(0, 12, 4):
                t.px(sx + 4 + k, sy + 7, METAL[3]); t.px(sx + 5 + k, sy + 6, METAL[3])
            if r.random() < .3:
                cx, cy = sx + r.randint(4, 11), sy + r.randint(4, 11)
                for _ in range(10):
                    t.px(cx + r.randint(-3, 3), cy + r.randint(-2, 2), scenes.RUST[1], 140)
    return t.im


def t_paver(seed, pal=PAVE, size=8, grass_gaps=True):
    t = Tex(seed); r = t.r
    t.rect(0, 0, T - 1, T - 1, GR[2] if grass_gaps else pal[1])
    for ty in range(0, T, size):
        off = size // 2 if (ty // size) % 2 else 0
        for tx in range(-size + off, T, size):
            if grass_gaps and r.random() < .08:
                continue
            c = r.choice(pal[:3])
            t.rect(tx, ty, tx + size - 2, ty + size - 2, c)
            t.rect(tx, ty, tx + size - 2, ty, tuple(min(255, v + 14) for v in c))
    return t.im


def t_white(seed):
    t = t_paver(seed, pal=WHITE, grass_gaps=False)
    return t


def t_plank(seed, vertical=False):
    t = Tex(seed)
    for k in range(0, T, 4):
        c = PLANK[0] if (k // 4) % 2 else PLANK[2]
        if vertical:
            t.rect(k, 0, k + 2, T - 1, c); t.rect(k + 3, 0, k + 3, T - 1, PLANK[3])
        else:
            t.rect(0, k, T - 1, k + 2, c); t.rect(0, k + 3, T - 1, k + 3, PLANK[3])
    for _ in range(6):
        t.px(t.r.randint(0, T), t.r.randint(0, T), PLANK[1])
    return t.im


def t_chasm(seed):
    t = Tex(seed); t.rect(0, 0, T - 1, T - 1, (18, 20, 28))
    for _ in range(6):
        y = t.r.randint(0, T)
        t.rect(0, y, T - 1, y, (60, 66, 78), 70)
    return t.im


def t_wall(seed, window=False, ivy=False):
    t = Tex(seed); r = t.r
    t.rect(0, 0, T - 1, T - 1, (96, 70, 58))
    for x in range(0, T, 4):
        t.rect(x, 0, x, T - 1, (78, 56, 48))
    for _ in range(3):
        x = r.randint(0, T); t.rect(x, r.randint(0, 10), x + 1, r.randint(14, 31), scenes.RUST[1])
    if window:
        t.rect(3, 4, 28, 26, (180, 210, 220))
        for k in (3, 15, 28):
            t.rect(k, 4, k, 26, (60, 50, 44))
        t.rect(3, 15, 28, 15, (60, 50, 44))
        if ivy:
            for xx in range(4, 28, 3):
                for k in range(r.randint(2, 12)):
                    t.px(xx + (k // 3) % 2, 4 + k, IVY[r.randint(0, 2)])
    return t.im


# ---------------------------------------------------------- corner Wang set
def wang(upper, lower, rim=None, rim_lower=None, seed=0, rough=4.0, rim_w=1):
    """16 tiles; upper/lower are lists of base tiles (variants). Membership via
    bilinear corner interpolation + noise that vanishes at tile borders, so any
    two tiles agreeing on a shared edge's corners match pixel-perfectly there."""
    tiles = []
    for idx in range(16):
        nw, ne, sw, se = (idx >> 3) & 1, (idx >> 2) & 1, (idx >> 1) & 1, idx & 1
        r = random.Random(seed * 100 + idx)
        A, B = upper[idx % len(upper)], lower[idx % len(lower)]
        ph = [r.random() * 6.28 for _ in range(4)]
        mem = [[0.0] * T for _ in range(T)]
        for y in range(T):
            for x in range(T):
                u, v = (x + .5) / T, (y + .5) / T
                b = nw * (1 - u) * (1 - v) + ne * u * (1 - v) + sw * (1 - u) * v + se * u * v
                n = (math.sin(x * .55 + ph[0]) * math.cos(y * .43 + ph[1]) +
                     .5 * math.sin(x * 1.3 + y * .9 + ph[2]))
                env = math.sin(math.pi * u) * math.sin(math.pi * v)
                mem[y][x] = b + n * env * rough / 16
        out = Image.new("RGBA", (T, T))
        for y in range(T):
            for x in range(T):
                out.putpixel((x, y), A.getpixel((x, y)) if mem[y][x] >= .5 else B.getpixel((x, y)))
        if rim:   # rim on the upper side of the boundary
            for y in range(T):
                for x in range(T):
                    if mem[y][x] >= .5:
                        near = False
                        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                            for k in range(1, rim_w + 1):
                                xx, yy = x + dx * k, y + dy * k
                                if 0 <= xx < T and 0 <= yy < T and mem[yy][xx] < .5:
                                    near = True
                        if near:
                            c = rim(x, y, r) if callable(rim) else rim
                            out.putpixel((x, y), tuple(c) + (255,))
        if rim_lower:
            for y in range(T):
                for x in range(T):
                    if mem[y][x] < .5 and any(0 <= x + dx < T and 0 <= y + dy < T and mem[y + dy][x + dx] >= .5
                                              for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))):
                        c = rim_lower(x, y, r) if callable(rim_lower) else rim_lower
                        out.putpixel((x, y), tuple(c) + (255,))
        tiles.append(out)
    return tiles


def chasm_wang(grass, seed=0):
    """Gorge edge: upper = chasm. Rock face band inside the chasm, darkening with depth."""
    tiles = []
    for idx in range(16):
        nw, ne, sw, se = (idx >> 3) & 1, (idx >> 2) & 1, (idx >> 1) & 1, idx & 1
        r = random.Random(seed + idx)
        ph = [r.random() * 6.28 for _ in range(3)]
        mem = [[0.0] * T for _ in range(T)]
        for y in range(T):
            for x in range(T):
                u, v = (x + .5) / T, (y + .5) / T
                b = nw * (1 - u) * (1 - v) + ne * u * (1 - v) + sw * (1 - u) * v + se * u * v
                n = math.sin(x * .6 + ph[0]) * math.cos(y * .5 + ph[1])
                mem[y][x] = b + n * math.sin(math.pi * u) * math.sin(math.pi * v) * .2
        G = grass[idx % len(grass)]
        out = Image.new("RGBA", (T, T))
        for y in range(T):
            for x in range(T):
                if mem[y][x] < .5:
                    out.putpixel((x, y), G.getpixel((x, y)))
                    continue
                d = (mem[y][x] - .5) * 2          # 0 at edge, 1 deep inside
                if d < .28:
                    c = ROCK[0] if (y + x // 3) % 7 else ROCK[3]
                    if d < .06:
                        c = ROCK[2]
                else:
                    t = min(1, (d - .28) / .5)
                    c = tuple(int(ROCK[1][i] * (1 - t) + (16, 18, 26)[i] * t) for i in range(3))
                out.putpixel((x, y), tuple(c) + (255,))
        tiles.append(out)
    return tiles


# --------------------------------------------------------- prop extraction
class Recorder(scenes.Scene):
    captured = []

    def render(self, *a, **k):
        Recorder.captured.append(self)
        return Image.new("RGBA", (scenes.W * scenes.UP, scenes.H * scenes.UP))


def extract_props(fn):
    for mod in (scenes, biomes, replicant_biomes):
        mod.Scene = Recorder
    Recorder.captured = []
    fn()
    props = []
    for sc in Recorder.captured:
        for y, f, glow in sc.items:
            q = f.__qualname__
            if "sprite" in q or "butterflies" in q:
                continue
            parts = [p for p in q.split(".") if p != "<locals>"]
            name = parts[-2] if len(parts) >= 2 else parts[0]
            if name.startswith(("scene_", "desert", "flooded", "factory", "park",
                                "cliff_village", "seafront", "build")):
                name = "decor"
            if name == "spawn" and f.__name__ == "g":
                name = "spawn_beam"
            canvas = Image.new("RGBA", (scenes.W, scenes.H), (0, 0, 0, 0))
            sc.im = canvas
            f()
            bb = canvas.getbbox()
            if not bb:
                continue
            crop = canvas.crop(bb)
            props.append({"name": name, "img": crop, "anchor_y": int(y) - bb[1],
                          "anchor_x": crop.width // 2})
    for mod in (scenes, biomes, replicant_biomes):
        mod.Scene = scenes.Scene.__mro__[0] if False else _ORIG_SCENE
    return props


_ORIG_SCENE = scenes.Scene


def dedupe(props, per_name=4):
    out, seen = [], {}
    for p in props:
        lst = seen.setdefault(p["name"], [])
        sz = p["img"].size
        if any(abs(sz[0] - s[0]) <= 2 and abs(sz[1] - s[1]) <= 2 for s in lst):
            continue
        if len(lst) >= per_name:
            continue
        lst.append(sz)
        p = dict(p); p["id"] = f'{p["name"]}_{len(lst)}'
        out.append(p)
    return out


def atlas(props, path, pad=4, maxw=1024):
    ims = [up(p["img"]) for p in props]
    x = y = rowh = 0
    pos = []
    for im in ims:
        if x + im.width > maxw:
            x, y, rowh = 0, y + rowh + pad, 0
        pos.append((x, y)); x += im.width + pad; rowh = max(rowh, im.height)
    sheet = Image.new("RGBA", (maxw, y + rowh), (0, 0, 0, 0))
    meta = {}
    for p, im, (px_, py_) in zip(props, ims, pos):
        sheet.paste(im, (px_, py_))
        meta[p["id"]] = {"rect": [px_, py_, im.width, im.height],
                         "anchor": [p["anchor_x"] * SC, min(p["anchor_y"] * SC, im.height)]}
    sheet.save(path + ".png")
    json.dump(meta, open(path + ".json", "w"), indent=1)
    return sheet


# ---------------------------------------------------------- spawn pad anim
def spawn_frames():
    frames, beams = [], []
    for k in range(4):
        sc = Recorder(9)
        sc.items = []
        sc.spawn(100, 120, 16, beam=True, t=k * 0.4)
        imgs = []
        for y, f, glow in sc.items:
            canvas = Image.new("RGBA", (scenes.W, scenes.H), (0, 0, 0, 0))
            sc.im = canvas; f()
            imgs.append(canvas)
        frames.append(imgs[0].crop((76, 104, 124, 132)))
        beams.append(imgs[1].crop((84, 0, 116, 124)))
    return frames, beams


# ---------------------------------------------------------- packing helpers
def tile_sheet(groups, path):
    """groups: list of (name, [tiles]) -> rows; returns meta"""
    cols = max(len(t) for _, t in groups)
    cols = min(cols, 8)
    rows_needed = sum((len(t) + cols - 1) // cols for _, t in groups)
    sheet = Image.new("RGBA", (cols * T * SC, rows_needed * T * SC), (0, 0, 0, 0))
    meta, row = {}, 0
    for name, tiles in groups:
        rects = []
        for i, tl in enumerate(tiles):
            c, rr = i % cols, row + i // cols
            sheet.paste(up(tl), (c * T * SC, rr * T * SC))
            rects.append([c * T * SC, rr * T * SC, T * SC, T * SC])
        meta[name] = rects
        row += (len(tiles) + cols - 1) // cols
    sheet.save(path + ".png")
    return sheet, meta


def demo_map(base_lo, base_hi, wset, props, path, cols=12, rows=7, seed=0, water_frames=None,
             prob=0.45, prop_count=10, label=""):
    r = random.Random(seed)
    corners = [[1 if r.random() < prob else 0 for _ in range(cols + 1)] for _ in range(rows + 1)]
    for _ in range(2):  # smooth
        nc = [[0] * (cols + 1) for _ in range(rows + 1)]
        for y in range(rows + 1):
            for x in range(cols + 1):
                s = n = 0
                for dy in (-1, 0, 1):
                    for dx in (-1, 0, 1):
                        if 0 <= y + dy <= rows and 0 <= x + dx <= cols:
                            s += corners[y + dy][x + dx]; n += 1
                nc[y][x] = 1 if s / n > .5 else 0
        corners = nc
    im = Image.new("RGBA", (cols * T, rows * T))
    for ty in range(rows):
        for tx in range(cols):
            idx = corners[ty][tx] * 8 + corners[ty][tx + 1] * 4 + corners[ty + 1][tx] * 2 + corners[ty + 1][tx + 1]
            if idx == 0:
                tl = r.choice(base_lo)
            elif idx == 15:
                tl = r.choice(base_hi)
            else:
                tl = wset[idx]
            im.paste(tl, (tx * T, ty * T))
    places = []
    lows = [(tx, ty) for ty in range(rows) for tx in range(cols)
            if corners[ty][tx] + corners[ty][tx + 1] + corners[ty + 1][tx] + corners[ty + 1][tx + 1] == 0]
    r.shuffle(lows)
    for p in sorted([r.choice(props) for _ in range(min(prop_count, len(lows)))] if props else [],
                    key=lambda p: 0):
        if not lows:
            break
        tx, ty = lows.pop()
        places.append((ty * T + T - 4, tx * T + T // 2, p))
    for y, x, p in sorted(places, key=lambda t: t[0]):
        im.alpha_composite(p["img"], (int(x - p["anchor_x"]), int(y - p["anchor_y"])) if
                           x - p["anchor_x"] >= 0 and y - p["anchor_y"] >= 0 else (max(0, int(x - p["anchor_x"])), max(0, int(y - p["anchor_y"]))))
    big = up(up(im)) if im.width < 500 else up(im)
    out = Image.new("RGB", (big.width, big.height + 30), (22, 26, 38))
    out.paste(big.convert("RGB"), (0, 0))
    ImageDraw.Draw(out).text((10, big.height + 9), label, fill=(210, 230, 235))
    out.save(path)


# ================================================================= BIOMES
def build():
    os.makedirs(OUT, exist_ok=True)
    summary = {}
    grass = [t_grass(i) for i in range(4)]
    biome_defs = []

    # ---------------------------------------------------------- meadow
    dirt = [t_dirt(10 + i) for i in range(4)]
    slab = [t_slab(20 + i) for i in range(4)]
    biome_defs.append(dict(
        key="meadow", title="Meadow ruins (spawn plaza, old road)",
        tiles=[("grass", grass), ("dirt", dirt), ("concrete_slab", slab)],
        wangs=[("concrete_slab__over__grass", wang(slab, grass, rim=CON[3], seed=1)),
               ("dirt__over__grass", wang(dirt, grass, rim_lower=GR[2], seed=2))],
        scenes=[scenes.scene_plaza, scenes.scene_forest, scenes.scene_night],
        demo=("concrete_slab__over__grass", grass, slab)))

    # ---------------------------------------------------------- dune
    sand = [t_sand(30 + i) for i in range(4)]
    water = [t_water(40, f) for f in range(4)]
    biome_defs.append(dict(
        key="dune_city", title="Dune City",
        tiles=[("sand", sand), ("oasis_grass", grass)],
        anim=[("pond_water", water)],
        wangs=[("oasis_grass__over__sand", wang(grass, sand, rim=(150, 160, 90), seed=3)),
               ("water__over__sand", wang(water[:1], sand, rim=WAT[2], rim_lower=SAND[1], seed=4))],
        scenes=[biomes.desert], demo=("oasis_grass__over__sand", sand, grass)))

    # ---------------------------------------------------------- drowned
    flood = [t_water(50 + v, 0) for v in range(2)]
    flood_anim = [t_water(50, f) for f in range(4)]
    roof = [t_slab(60 + i) for i in range(4)]
    planks_h = [t_plank(70), t_plank(71)]
    planks_v = [t_plank(72, True), t_plank(73, True)]
    biome_defs.append(dict(
        key="drowned_district", title="Drowned District",
        tiles=[("flood_water", flood), ("rooftop_concrete", roof),
               ("plank_bridge_h", planks_h), ("plank_bridge_v", planks_v)],
        anim=[("flood_water_anim", flood_anim)],
        wangs=[("rooftop__over__water", wang(roof, flood, rim=scenes.WALL[2], rim_lower=WAT[3],
                                             seed=5, rough=1.5, rim_w=2))],
        scenes=[biomes.flooded], demo=("rooftop__over__water", flood, roof)))

    # ---------------------------------------------------------- rust works
    metal = [t_metal(80 + i) for i in range(4)]
    walls = [t_wall(90), t_wall(91), t_wall(92, window=True), t_wall(93, window=True, ivy=True)]
    biome_defs.append(dict(
        key="rust_works", title="Rust Works",
        tiles=[("metal_floor", metal), ("back_wall", walls), ("overgrowth", grass)],
        wangs=[("overgrowth__over__metal", wang(grass, metal, rim=IVY[0], seed=6))],
        scenes=[biomes.factory], demo=("overgrowth__over__metal", metal, grass)))

    # ---------------------------------------------------------- last fair
    pavers = [t_paver(100 + i) for i in range(4)]
    biome_defs.append(dict(
        key="last_fair", title="The Last Fair",
        tiles=[("pavers", pavers), ("lawn", grass)],
        wangs=[("pavers__over__lawn", wang(pavers, grass, rim=PAVE[1], seed=7))],
        scenes=[biomes.park], demo=("pavers__over__lawn", grass, pavers)))

    # ---------------------------------------------------------- gorge village
    deck = [t_plank(110), t_plank(111), t_plank(112, True), t_plank(113, True)]
    chasm = [t_chasm(120 + i) for i in range(2)]
    biome_defs.append(dict(
        key="gorge_village", title="Gorge Village",
        tiles=[("grass", grass), ("wood_deck", deck), ("chasm_deep", chasm)],
        wangs=[("chasm__over__grass", chasm_wang(grass, seed=8))],
        scenes=[replicant_biomes.cliff_village], demo=("chasm__over__grass", grass, chasm)))

    # ---------------------------------------------------------- white harbour
    prom = [t_white(130 + i) for i in range(4)]
    hsand = [t_sand(140 + i, pal=HSAND) for i in range(4)]
    sea = [t_water(150, f, pal=(SEA[1], SEA[0], SEA[2])) for f in range(4)]
    biome_defs.append(dict(
        key="white_harbour", title="White Harbour",
        tiles=[("promenade", prom), ("beach_sand", hsand)],
        anim=[("sea", sea)],
        wangs=[("promenade__over__sand", wang(prom, hsand, rim=WHITE[3], seed=9)),
               ("sea__over__sand", wang(sea[:1], hsand, rim=SEA[3], rim_lower=SEA[2], seed=10, rim_w=2))],
        scenes=[replicant_biomes.seafront], demo=("sea__over__sand", hsand, sea[:1])))

    spawn_pad, spawn_beam = spawn_frames()

    for b in biome_defs:
        d = os.path.join(OUT, b["key"]); os.makedirs(d, exist_ok=True)
        groups = list(b["tiles"]) + [(n, t) for n, t in b.get("anim", [])] + list(b["wangs"])
        sheet, tmeta = tile_sheet(groups, os.path.join(d, f"{b['key']}_tiles"))
        info = {"tile_size_px": T * SC, "native_tile": T, "tiles": tmeta,
                "wang_index": "NW*8+NE*4+SW*2+SE*1, bit=1 means the first-named (upper) terrain",
                "animated": [n for n, _ in b.get("anim", [])], "anim_fps": 4}
        json.dump(info, open(os.path.join(d, f"{b['key']}_tiles.json"), "w"), indent=1)

        props = []
        for fn in b["scenes"]:
            props += extract_props(fn)
        props = [p for p in props if p["name"] not in ("spawn", "spawn_beam")]
        props = dedupe(props)
        atlas(props, os.path.join(d, f"{b['key']}_props"))
        # per-prop PNGs
        pd = os.path.join(d, "props"); os.makedirs(pd, exist_ok=True)
        for p in props:
            up(p["img"]).save(os.path.join(pd, p["id"] + ".png"))

        wname, lo, hi = b["demo"]
        wset = dict(b["wangs"])[wname]
        demo_map(lo, hi, wset, [p for p in props if p["img"].height < 120], os.path.join(d, f"{b['key']}_demo_map.png"),
                 seed=len(b["key"]), label=f"{b['title']} - demo map built only from this pack's tiles + props")
        summary[b["key"]] = {"tiles": {k: len(v) for k, v in tmeta.items()},
                             "props": len(props)}

    # shared: spawn pad (animated) + beam
    d = os.path.join(OUT, "shared"); os.makedirs(d, exist_ok=True)
    pad = Image.new("RGBA", (48 * SC * 4, 28 * SC)); beam = Image.new("RGBA", (32 * SC * 4, 124 * SC))
    for k in range(4):
        pad.paste(up(spawn_pad[k]), (k * 48 * SC, 0)); beam.paste(up(spawn_beam[k]), (k * 32 * SC, 0))
    pad.save(os.path.join(d, "spawn_pad_anim.png")); beam.save(os.path.join(d, "spawn_beam_anim.png"))
    json.dump({"spawn_pad_anim": {"frame": [48 * SC, 28 * SC], "frames": 4, "fps": 6,
                                  "anchor": [24 * SC, 16 * SC]},
               "spawn_beam_anim": {"frame": [32 * SC, 124 * SC], "frames": 4, "fps": 6,
                                   "anchor": [16 * SC, 116 * SC], "blend": "additive or alpha, draw after pad"}},
              open(os.path.join(d, "shared.json"), "w"), indent=1)
    json.dump(summary, open(os.path.join(OUT, "summary.json"), "w"), indent=1)
    return summary


if __name__ == "__main__":
    s = build()
    for k, v in s.items():
        print(k, v)
