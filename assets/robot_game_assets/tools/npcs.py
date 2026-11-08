"""Robot NPC sprite sheets matching the TV-robot hero.

Drawn on a 48x48 native grid (robot box 32x32 at (8,8)), outlined 1px, then
scaled x2 -> 96x96 cells, feet at (48,78) like the hero sheet.
Usage: python3 npcs.py <outdir>
"""
import json
import sys
from PIL import Image, ImageDraw

OUT = sys.argv[1]
N, B, SCALE = 48, 8, 2

P = {
    "a": (11, 11, 15), "#": (50, 62, 79), "b": (17, 128, 113), "c": (29, 168, 149),
    "d": (207, 255, 112), "e": (39, 39, 54), "f": (15, 95, 115), "g": (150, 66, 83),
    "h": (255, 140, 140), "i": (235, 86, 75), "j": (33, 31, 52), "k": (31, 102, 153),
    "w": (244, 244, 244),
    # extras for NPC variety
    "l": (150, 165, 185),  # light steel
    "m": (100, 112, 135),  # mid steel
    "y": (255, 205, 117),  # yellow
    "o": (214, 124, 58),   # orange / rust
    "r": (130, 70, 45),    # dark rust / wood
    "t": (92, 52, 36),     # darker wood
    "v": (110, 190, 80),   # leaf
    "n": (45, 115, 70),    # dark leaf
}


class C:
    def __init__(self):
        self.im = Image.new("RGBA", (N, N), (0, 0, 0, 0))
        self.fx = Image.new("RGBA", (N, N), (0, 0, 0, 0))

    def r(self, x0, y0, x1, y1, col, fx=False):
        im = self.fx if fx else self.im
        for y in range(min(y0, y1), max(y0, y1) + 1):
            for x in range(min(x0, x1), max(x0, x1) + 1):
                if 0 <= x + B < N and 0 <= y + B < N:
                    im.putpixel((x + B, y + B), P[col] + (255,))

    def p(self, x, y, col, fx=False):
        self.r(x, y, x, y, col, fx)

    def render(self):
        src = self.im
        out = Image.new("RGBA", (N, N), (0, 0, 0, 0))
        op = lambda x, y: 0 <= x < N and 0 <= y < N and src.getpixel((x, y))[3]
        for y in range(N):
            for x in range(N):
                if not op(x, y) and any(op(x + dx, y + dy) for dx, dy in
                                        ((1, 0), (-1, 0), (0, 1), (0, -1))):
                    out.putpixel((x, y), P["a"] + (255,))
        out.alpha_composite(src)
        out.alpha_composite(self.fx)
        return out.resize((N * SCALE, N * SCALE), Image.NEAREST)


def legs(c, xl, xr, y0, y1, w, lift, cols, far=None):
    """Two legs; lift=(left,right) px raised. cols = (main, shade)."""
    for i, (x, l) in enumerate(((xl, lift[0]), (xr, lift[1]))):
        main, shade = cols if far != i else (cols[1], cols[1])
        c.r(x, y0, x + w - 1, y1 - l, main)
        c.r(x + w - 1, y0, x + w - 1, y1 - l, shade)


WALK_LIFT = [(0, 0), (2, 0), (0, 0), (0, 2)]
WALK_BOB = [0, 1, 0, 1]
STRIDE = [(-2, 2), (0, 0), (2, -2), (0, 0)]  # side view leg x offsets (far, near)


# ===================================================================== SCRAP
def scrap(c, view, ph, bob, talk=False, blink=False):
    """Trash-can bot on treads. ph: walk phase 0..3 or None."""
    y = bob
    tread = (ph or 0) % 2
    if view in ("down", "up"):
        c.r(7, 27, 24, 30, "#")
        for x in range(7 + tread, 25, 2):
            c.p(x, 28, "j"); c.p(x, 30, "j")
        c.r(7, 27, 24, 27, "m")
        c.r(9, 13 + y, 22, 26, "o")
        c.r(9, 13 + y, 10, 26, "y")
        c.r(21, 13 + y, 22, 26, "r")
        for yy in (16 + y, 23):
            c.r(9, yy, 22, yy, "r")
        lid = -2 if talk else 0
        c.r(8, 10 + y + lid, 23, 12 + y + lid, "l")
        c.r(8, 12 + y + lid, 23, 12 + y + lid, "m")
        c.r(14, 8 + y + lid, 17, 9 + y + lid, "m")
        if talk:  # junk peeking out
            c.r(11, 11 + y, 13, 12 + y, "v"); c.r(18, 11 + y, 19, 12 + y, "k")
        arm_y = 19 + y + (1 if ph in (1,) else 0)
        c.r(6, arm_y, 8, arm_y + 2, "m"); c.r(5, arm_y + 2, 6, arm_y + 3, "#")
        c.r(23, arm_y, 25, arm_y + 2, "m"); c.r(25, arm_y + 2, 26, arm_y + 3, "#")
        if view == "down":
            c.r(12, 17 + y, 19, 21 + y, "e")
            if blink:
                c.r(13, 19 + y, 18, 19 + y, "d")
            else:
                c.r(13, 18 + y, 18, 20 + y, "d")
                c.r(15, 18 + y, 16, 20 + y, "e")
        else:
            c.r(13, 18 + y, 18, 21 + y, "r")
            for xx in (14, 17):
                c.p(xx, 19 + y, "y")
    else:  # right
        c.r(9, 27, 23, 30, "#")
        for x in range(9 + tread, 24, 2):
            c.p(x, 28, "j"); c.p(x, 30, "j")
        c.r(9, 27, 23, 27, "m")
        c.r(11, 13 + y, 21, 26, "o")
        c.r(11, 13 + y, 12, 26, "r")
        c.r(11, 16 + y, 21, 16 + y, "r"); c.r(11, 23, 21, 23, "r")
        lid = -2 if talk else 0
        c.r(10, 10 + y + lid, 22, 12 + y + lid, "l")
        c.r(10, 12 + y + lid, 22, 12 + y + lid, "m")
        c.r(19, 17 + y, 22, 21 + y, "e")
        if blink:
            c.r(20, 19 + y, 22, 19 + y, "d")
        else:
            c.r(20, 18 + y, 22, 20 + y, "d")
        arm = [0, 1, 0, -1][ph or 0]
        c.r(14 + arm, 19 + y, 17 + arm, 21 + y, "m")
        c.r(17 + arm, 21 + y, 18 + arm, 22 + y, "#")


# ===================================================================== RADIO
def radio(c, view, ph, bob, talk=False, blink=False):
    """Lanky bot with a vintage radio for a head."""
    y = bob
    lift = WALK_LIFT[ph] if ph is not None else (0, 0)
    tip = "i" if not blink else "g"
    if view in ("down", "up"):
        legs(c, 12, 18, 21, 30, 2, lift, ("m", "#"))
        c.r(11, 30 - lift[0], 14, 30 - lift[0], "#")
        c.r(17, 30 - lift[1], 20, 30 - lift[1], "#")
        c.r(12, 14 + y, 19, 21 + y, "l")
        c.r(12, 20 + y, 19, 21 + y, "m")
        sw = [0, 1, 0, -1][ph] if ph is not None else 0
        c.r(10, 15 + y, 11, 21 + y + sw, "m"); c.r(9, 21 + y + sw, 11, 22 + y + sw, "#")
        c.r(20, 15 + y, 21, 21 + y - sw, "m"); c.r(20, 21 + y - sw, 22, 22 + y - sw, "#")
        c.r(15, 10 + y, 16, 13 + y, "#")
        c.r(8, 1 + y, 23, 9 + y, "r")
        c.r(8, 1 + y, 23, 1 + y, "o")
        c.r(8, 9 + y, 23, 9 + y, "t")
        c.r(20, -5 + y, 20, 0 + y, "m"); c.p(20, -6 + y, tip)
        if view == "down":
            g1, g2 = ("y", "o") if talk else ("t", "r")
            c.r(10, 3 + y, 16, 7 + y, g2)
            for yy in range(3, 8, 2):
                for xx in range(10, 17, 2):
                    c.p(xx, yy + y, g1)
            c.r(18, 3 + y, 21, 7 + y, "y")
            c.r(19, 4 + y, 20, 6 + y, "e" if not blink else "y")
            c.p(20, 5 + y, "d")
            c.r(14, 16 + y, 17, 18 + y, "k")
        else:
            c.r(10, 3 + y, 21, 7 + y, "t")
            c.r(12, 4 + y, 19, 6 + y, "r")
            c.r(15, 15 + y, 16, 19 + y, "m")
    else:
        dx = STRIDE[ph] if ph is not None else (0, 0)
        lf = WALK_LIFT[ph] if ph is not None else (0, 0)
        c.r(14 + dx[0], 21, 15 + dx[0], 30 - lf[1], "#")
        c.r(13 + dx[0], 30 - lf[1], 16 + dx[0], 30 - lf[1], "j")
        c.r(13, 14 + y, 18, 21 + y, "l")
        c.r(13, 14 + y, 13, 21 + y, "m")
        c.r(16 + dx[1], 21, 17 + dx[1], 30 - lf[0], "m")
        c.r(15 + dx[1], 30 - lf[0], 18 + dx[1], 30 - lf[0], "#")
        c.r(15, 10 + y, 16, 13 + y, "#")
        c.r(11, 1 + y, 20, 9 + y, "r")
        c.r(11, 1 + y, 20, 1 + y, "o")
        c.r(11, 9 + y, 20, 9 + y, "t")
        c.r(19, 3 + y, 20, 7 + y, "y")
        c.r(13, 4 + y, 17, 6 + y, "t")
        c.r(13, -5 + y, 13, 0 + y, "m"); c.p(13, -6 + y, tip)
        sw = [-2, 0, 2, 0][ph] if ph is not None else 0
        c.r(15, 15 + y, 16, 18 + y, "m")
        c.r(15 + sw // 2, 19 + y, 16 + sw // 2, 21 + y, "m")
        c.r(15 + sw // 2, 22 + y, 17 + sw // 2, 22 + y, "#")


# ================================================================== MERCHANT
def merchant(c, view, ph, bob, talk=False, blink=False):
    """Vending-machine shopkeeper on stubby legs."""
    y = bob
    lift = WALK_LIFT[ph] if ph is not None else (0, 0)
    if view in ("down", "up"):
        legs(c, 10, 18, 28, 31, 4, lift, ("#", "j"))
        c.r(6, 3 + y, 25, 27, "l")
        c.r(6, 3 + y, 7, 27, "w")
        c.r(24, 3 + y, 25, 27, "m")
        c.r(6, 3 + y, 25, 3 + y, "w")
        c.r(6, 26, 25, 27, "m")
        if view == "down":
            c.r(9, 5 + y, 22, 8 + y, "e")
            if talk:
                for xx in (12, 13, 18, 19):
                    c.p(xx, 6 + y, "d")
                c.p(11, 7 + y, "d"); c.p(14, 7 + y, "d"); c.p(17, 7 + y, "d"); c.p(20, 7 + y, "d")
            elif blink:
                c.r(11, 7 + y, 14, 7 + y, "d"); c.r(17, 7 + y, 20, 7 + y, "d")
            else:
                c.r(12, 6 + y, 13, 7 + y, "d"); c.r(18, 6 + y, 19, 7 + y, "d")
            c.r(9, 10 + y, 19, 22, "k")
            c.r(9, 10 + y, 19, 10 + y, "m")
            goods = ["i", "y", "v", "h", "o", "d"]
            for row, yy in enumerate((12 + y, 16 + y, 20)):
                c.r(9, yy + 1, 19, yy + 1, "m")
                for n, xx in enumerate((10, 13, 16)):
                    c.r(xx, yy - 1, xx + 1, yy, goods[(row * 3 + n + (1 if talk else 0)) % 6])
            c.r(21, 11 + y, 22, 14 + y, "e"); c.r(21, 12 + y, 22, 12 + y, "y")
            c.r(21, 17, 22, 18, "d" if not blink else "w")
            c.r(9, 24, 19, 25, "e")
            sw = [0, 1, 0, -1][ph] if ph is not None else 0
            c.r(3, 12 + y, 5, 18 + y + sw, "m"); c.r(3, 19 + y + sw, 5, 20 + y + sw, "#")
            c.r(26, 12 + y, 28, 18 + y - sw, "m"); c.r(26, 19 + y - sw, 28, 20 + y - sw, "#")
        else:
            for yy in range(7 + y, 24, 3):
                c.r(10, yy, 21, yy, "m")
            c.r(14, 22, 17, 25, "#"); c.r(15, 23, 16, 24, "y")
            c.r(3, 12 + y, 5, 18 + y, "m"); c.r(26, 12 + y, 28, 18 + y, "m")
            c.r(3, 19 + y, 5, 20 + y, "#"); c.r(26, 19 + y, 28, 20 + y, "#")
    else:
        dx = STRIDE[ph] if ph is not None else (0, 0)
        lf = WALK_LIFT[ph] if ph is not None else (0, 0)
        c.r(12 + dx[0], 28, 15 + dx[0], 31 - lf[1], "j")
        c.r(9, 3 + y, 22, 27, "l")
        c.r(9, 3 + y, 10, 27, "m")
        c.r(9, 3 + y, 22, 3 + y, "w")
        c.r(9, 26, 22, 27, "m")
        c.r(21, 5 + y, 22, 8 + y, "e")
        c.p(22, 6 + y, "d" if not blink else "e")
        c.r(21, 10 + y, 22, 22, "k")
        for yy in range(7 + y, 24, 4):
            c.r(12, yy, 18, yy, "m")
        c.r(16 + dx[1], 28, 19 + dx[1], 31 - lf[0], "#")
        sw = [-2, 0, 2, 0][ph] if ph is not None else 0
        c.r(15, 12 + y, 17, 16 + y, "m")
        c.r(15 + sw, 17 + y, 17 + sw, 19 + y, "m")
        c.r(15 + sw, 20 + y, 17 + sw, 21 + y, "#")


# ================================================================== GARDENER
def gardener(c, view, ph, bob, talk=False, blink=False):
    """Flower-pot bot with a glass dome and a sprout."""
    y = bob
    lift = WALK_LIFT[ph] if ph is not None else (0, 0)
    sway = [0, 1, 0, -1][ph] if ph is not None else (1 if bob else 0)
    if view in ("down", "up"):
        legs(c, 11, 18, 25, 30, 3, lift, ("m", "#"))
        c.r(10, 30 - lift[0], 14, 30 - lift[0], "#")
        c.r(17, 30 - lift[1], 21, 30 - lift[1], "#")
        c.r(9, 15 + y, 22, 17 + y, "i")
        c.r(9, 15 + y, 22, 15 + y, "h")
        c.r(10, 18 + y, 21, 25, "g")
        c.r(10, 18 + y, 11, 25, "i")
        sw = [0, 1, 0, -1][ph] if ph is not None else 0
        c.r(7, 18 + y, 8, 22 + y + sw, "m"); c.r(6, 22 + y + sw, 8, 23 + y + sw, "#")
        c.r(23, 18 + y, 24, 22 + y - sw, "m"); c.r(23, 22 + y - sw, 25, 23 + y - sw, "#")
        if view == "down":
            c.r(12, 19 + y, 19, 22 + y, "e")
            if blink:
                c.r(13, 21 + y, 14, 21 + y, "d"); c.r(17, 21 + y, 18, 21 + y, "d")
            else:
                c.r(13, 20 + y, 14, 21 + y, "d"); c.r(17, 20 + y, 18, 21 + y, "d")
            if talk:
                c.r(15, 22 + y, 16, 22 + y, "d")
        else:
            c.r(14, 19 + y, 17, 23 + y, "r")
        # glass dome
        c.r(10, 5 + y, 21, 14 + y, "c")
        c.r(11, 4 + y, 20, 4 + y, "c")
        c.r(11, 6 + y, 12, 9 + y, "w")
        c.r(10, 13 + y, 21, 14 + y, "r")
        # sprout
        c.r(15, 7 + y, 16, 12 + y, "n")
        c.r(12 + sway, 7 + y, 14 + sway, 8 + y, "v")
        c.r(17 + sway, 5 + y, 19 + sway, 6 + y, "v")
        if talk:
            c.r(14, 3 + y, 17, 3 + y, "y", fx=True)
    else:
        dx = STRIDE[ph] if ph is not None else (0, 0)
        lf = WALK_LIFT[ph] if ph is not None else (0, 0)
        c.r(13 + dx[0], 25, 15 + dx[0], 30 - lf[1], "#")
        c.r(12 + dx[0], 30 - lf[1], 16 + dx[0], 30 - lf[1], "j")
        c.r(11, 15 + y, 20, 17 + y, "i")
        c.r(11, 15 + y, 20, 15 + y, "h")
        c.r(12, 18 + y, 19, 25, "g")
        c.r(18, 19 + y, 19, 22 + y, "e")
        c.p(19, 20 + y, "d" if not blink else "e")
        c.r(16 + dx[1], 25, 18 + dx[1], 30 - lf[0], "m")
        c.r(15 + dx[1], 30 - lf[0], 19 + dx[1], 30 - lf[0], "#")
        c.r(12, 5 + y, 19, 14 + y, "c")
        c.r(13, 4 + y, 18, 4 + y, "c")
        c.r(17, 6 + y, 18, 9 + y, "w")
        c.r(12, 13 + y, 19, 14 + y, "r")
        c.r(15, 7 + y, 16, 12 + y, "n")
        c.r(12 + sway, 7 + y, 14 + sway, 8 + y, "v")
        c.r(17 + sway, 5 + y, 18 + sway, 6 + y, "v")
        sw = [-2, 0, 2, 0][ph] if ph is not None else 0
        c.r(15, 18 + y, 16, 20 + y, "m")
        c.r(15 + sw // 2, 21 + y, 16 + sw // 2, 22 + y, "m")
        c.r(15 + sw // 2, 23 + y, 17 + sw // 2, 23 + y, "#")


# ================================================================ assemble
def frame(fn, view, ph=None, bob=0, **kw):
    c = C()
    fn(c, view, ph, bob, **kw)
    return c.render()


def mirror(im):
    m = im.transpose(Image.FLIP_LEFT_RIGHT)
    out = Image.new("RGBA", im.size, (0, 0, 0, 0))
    out.paste(m, (-SCALE, 0))  # keep x-centre aligned (box centre 15.5 native)
    return out


def build(fn):
    rows = []
    for view in ("down", "up", "left", "right"):
        v = "right" if view == "left" else view
        fr = [frame(fn, v, None, 0), frame(fn, v, None, 1, blink=True)]
        rows.append((f"idle_{view}", [mirror(f) for f in fr] if view == "left" else fr))
    for view in ("down", "up", "left", "right"):
        v = "right" if view == "left" else view
        fr = [frame(fn, v, ph, WALK_BOB[ph]) for ph in range(4)]
        rows.append((f"walk_{view}", [mirror(f) for f in fr] if view == "left" else fr))
    rows.append(("talk_down", [frame(fn, "down", None, 0, talk=True),
                               frame(fn, "down", None, 1)]))
    return rows


FPS = {"idle": 2, "walk": 8, "talk": 6}
NPCS = {"scrap": scrap, "radio": radio, "merchant": merchant, "gardener": gardener}
CELL = N * SCALE
all_rows = {}
for name, fn in NPCS.items():
    rows = build(fn)
    all_rows[name] = rows
    cols = max(len(f) for _, f in rows)
    sheet = Image.new("RGBA", (cols * CELL, len(rows) * CELL), (0, 0, 0, 0))
    meta = {"image": f"npc_{name}.png", "frame_size": [CELL, CELL],
            "origin_feet": [48, 78], "animations": {}}
    for r, (an, frames) in enumerate(rows):
        for ci, im in enumerate(frames):
            sheet.paste(im, (ci * CELL, r * CELL))
        meta["animations"][an] = {"row": r, "frames": len(frames),
                                  "fps": FPS[an.split("_")[0]], "loop": True,
                                  "rects": [[ci * CELL, r * CELL, CELL, CELL]
                                            for ci in range(len(frames))]}
    sheet.save(f"{OUT}/npc_{name}.png")
    json.dump(meta, open(f"{OUT}/npc_{name}.json", "w"), indent=1)

# preview: one column per NPC, cycling through all its animations
S = 2
names = list(NPCS)
seq = ["idle_down", "walk_down", "walk_right", "walk_up", "walk_left", "talk_down"]
gif = []
for t in range(len(seq) * 16):
    an = seq[t // 16]
    cv = Image.new("RGBA", (len(names) * CELL * S, CELL * S + 28), (58, 64, 82, 255))
    d = ImageDraw.Draw(cv)
    for i, name in enumerate(names):
        frames = dict(all_rows[name])[an]
        k = an.split("_")[0]
        idx = (t * FPS[k] // 12) % len(frames)
        cv.alpha_composite(frames[idx].resize((CELL * S, CELL * S), Image.NEAREST), (i * CELL * S, 0))
        d.text((i * CELL * S + 6, CELL * S + 2), name, fill=(235, 235, 240, 255))
    d.text((6, CELL * S + 14), an, fill=(180, 190, 200, 255))
    gif.append(cv.convert("RGB"))
gif[0].save(f"{OUT}/npc_preview.gif", save_all=True, append_images=gif[1:],
            duration=1000 // 12, loop=0)

# lineup with the hero for scale check
try:
    hero = Image.open(f"{OUT}/../out2/tv_robot_sheet.png").crop((0, 0, CELL, CELL))
    lu = Image.new("RGBA", (CELL * 5, CELL), (120, 126, 140, 255))
    lu.alpha_composite(hero, (0, 0))
    for i, name in enumerate(names):
        lu.alpha_composite(dict(all_rows[name])["idle_down"][0], ((i + 1) * CELL, 0))
    lu.resize((lu.width * 2, lu.height * 2), Image.NEAREST).save(f"{OUT}/npc_lineup.png")
except FileNotFoundError:
    pass
print("ok")
