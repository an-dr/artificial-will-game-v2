"""Human characters (later turned into spirits) in the TV-robot style.

48x48 native grid (32x32 box at (8,8)), 1px outline, x2 -> 96x96 cells,
feet at (48,78). Rows: idle x4 dirs, walk x4 dirs.
Usage: python3 humans.py <outdir>
"""
import json
import sys
from dataclasses import dataclass
from PIL import Image

OUT = sys.argv[1]
N, BX, SCALE = 48, 8, 2
CELL = N * SCALE
G = 31

P = {
    "a": (11, 11, 15), "#": (50, 62, 79), "j": (33, 31, 52), "w": (244, 244, 244),
    "l": (150, 165, 185), "m": (100, 112, 135), "o": (214, 124, 58), "r": (130, 70, 45),
    "i": (235, 86, 75), "y": (255, 205, 117), "B": (150, 95, 55), "D": (95, 60, 40),
    "T": (225, 190, 140), "S": (240, 200, 160), "s": (200, 150, 115), "k": (31, 102, 153),
    "n": (45, 115, 70), "v": (110, 190, 80), "g": (150, 66, 83), "q": (90, 100, 60),
    "Q": (130, 140, 85), "W": (200, 205, 210),
}


class C:
    def __init__(self):
        self.im = Image.new("RGBA", (N, N), (0, 0, 0, 0))

    def r(self, x0, y0, x1, y1, col):
        for y in range(min(y0, y1), max(y0, y1) + 1):
            for x in range(min(x0, x1), max(x0, x1) + 1):
                if 0 <= x + BX < N and 0 <= y + BX < N:
                    self.im.putpixel((x + BX, y + BX), P[col] + (255,))

    def p(self, x, y, col):
        self.r(x, y, x, y, col)

    def render(self):
        src = self.im
        out = Image.new("RGBA", (N, N), (0, 0, 0, 0))
        op = lambda x, y: 0 <= x < N and 0 <= y < N and src.getpixel((x, y))[3]
        for y in range(N):
            for x in range(N):
                if not op(x, y) and any(op(x + a, y + b) for a, b in
                                        ((1, 0), (-1, 0), (0, 1), (0, -1))):
                    out.putpixel((x, y), P["a"] + (255,))
        out.alpha_composite(src)
        return out.resize((CELL, CELL), Image.NEAREST)


@dataclass
class H:
    name: str
    top: str; top_s: str; bottom: str; hair: str
    skin: str = "S"
    leg: int = 7; torso: int = 9; head: int = 7
    hair_style: str = "short"   # short | long | bald | pigtails
    hat: str = ""               # helmet | hardhat | straw | cap
    beard: bool = False
    glasses: bool = False
    coat: bool = False          # long coat / robe over legs
    dress: bool = False
    staff: bool = False
    pack: bool = False
    shoes: str = "j"


def human(c, h, view, ph=None, bob=0, blink=False):
    cx = 16
    lift = [(0, 0), (1, 0), (0, 0), (0, 1)][ph] if ph is not None else (0, 0)
    sw = [1, 0, -1, 0][ph] if ph is not None else 0
    lt = G - h.leg + 1                 # leg top
    tt = lt - h.torso                  # torso top
    ht = tt - h.head + bob             # head top
    long_lower = h.coat or h.dress

    if view in ("down", "up"):
        # legs
        for i, x in enumerate((cx - 3, cx + 1)):
            c.r(x, lt, x + 1, G - lift[i], h.bottom)
            c.r(x, G - lift[i], x + 1, G - lift[i], h.shoes)
        if h.pack and view == "up":
            pass
        # torso
        c.r(cx - 4, tt, cx + 3, lt - 1, h.top)
        c.r(cx + 2, tt, cx + 3, lt - 1, h.top_s)
        if long_lower:
            w = 5 if h.dress else 4
            c.r(cx - w, lt - 1, cx + w - 1, lt + 3, h.top)
            c.r(cx + w - 2, lt - 1, cx + w - 1, lt + 3, h.top_s)
        if h.coat and view == "down":
            c.r(cx - 1, tt + 1, cx, lt + 3, "W" if h.top == "w" else h.top_s)
        if h.pack:
            if view == "up":
                c.r(cx - 3, tt + 1, cx + 2, lt - 2, "q"); c.r(cx - 3, tt + 1, cx + 2, tt + 1, "Q")
            else:
                c.p(cx - 3, tt, "q"); c.p(cx + 2, tt, "q")
        # arms
        for i, (x, s) in enumerate(((cx - 6, sw), (cx + 4, -sw))):
            c.r(x, tt + 1, x + 1, lt - 2 + s, h.top_s if i else h.top)
            c.r(x, lt - 1 + s, x + 1, lt - 1 + s, h.skin)
        if h.staff:
            sx = cx + 6
            c.r(sx, tt - 4, sx, G, "D"); c.r(sx - 1, tt - 5, sx + 1, tt - 4, "B")
        # head
        c.r(cx - 3, ht + 1, cx + 2, tt - 1, h.skin)
        c.r(cx - 4, ht + 2, cx + 3, tt - 2, h.skin)
        if view == "up":
            if h.hair_style != "bald":
                c.r(cx - 4, ht + 1, cx + 3, tt - 2, h.hair)
                c.r(cx - 3, ht, cx + 2, ht, h.hair)
            if h.hair_style == "long":
                c.r(cx - 4, tt - 2, cx + 3, tt + 2, h.hair)
        else:
            ey = ht + 4
            if h.hair_style != "bald":
                c.r(cx - 3, ht, cx + 2, ht + 1, h.hair)
                c.r(cx - 4, ht + 1, cx - 4, ht + 3, h.hair)
                c.r(cx + 3, ht + 1, cx + 3, ht + 3, h.hair)
            if h.hair_style == "long":
                c.r(cx - 5, ht + 2, cx - 4, tt + 2, h.hair)
                c.r(cx + 3, ht + 2, cx + 4, tt + 2, h.hair)
            eye = "s" if blink else "a"
            c.p(cx - 2, ey, eye); c.p(cx + 1, ey, eye)
            if h.glasses:
                c.r(cx - 3, ey - 1, cx - 1, ey - 1, "j"); c.r(cx, ey - 1, cx + 2, ey - 1, "j")
                c.p(cx - 3, ey, "j"); c.p(cx + 2, ey, "j")
            if h.beard:
                c.r(cx - 3, ey + 1, cx + 2, tt + 2, "W")
            else:
                c.r(cx - 1, ey + 2, cx, ey + 2, "s")
        if h.hair_style == "pigtails":
            c.r(cx - 6, ht + 2, cx - 5, ht + 4, h.hair); c.r(cx + 4, ht + 2, cx + 5, ht + 4, h.hair)
        hat(c, h, cx, ht, front=True)
    else:  # right
        dx = [(-2, 2), (0, 0), (2, -2), (0, 0)][ph] if ph is not None else (0, 0)
        lf = lift
        c.r(cx - 1 + dx[0], lt, cx + dx[0], G - lf[1], h.bottom if not long_lower else h.top_s)
        c.r(cx - 1 + dx[0], G - lf[1], cx + 1 + dx[0], G - lf[1], "j")
        if h.pack:
            c.r(cx - 4, tt + 1, cx - 3, lt - 2, "q")
        c.r(cx - 2, tt, cx + 2, lt - 1, h.top)
        if long_lower:
            c.r(cx - 3, lt - 1, cx + 2 + (1 if h.dress else 0), lt + 3, h.top)
        c.r(cx + dx[1], lt, cx + 1 + dx[1], G - lf[0], h.bottom if not long_lower else h.top)
        c.r(cx + dx[1], G - lf[0], cx + 2 + dx[1], G - lf[0], h.shoes)
        # head
        c.r(cx - 2, ht + 1, cx + 2, tt - 1, h.skin)
        c.r(cx + 3, ht + 3, cx + 3, ht + 4, h.skin)
        if h.hair_style != "bald":
            c.r(cx - 2, ht, cx + 2, ht + 1, h.hair)
            c.r(cx - 3, ht + 1, cx - 1, ht + 4, h.hair)
        if h.hair_style == "long":
            c.r(cx - 3, ht + 1, cx - 2, tt + 3, h.hair)
        if h.hair_style == "pigtails":
            c.r(cx - 5, ht + 2, cx - 4, ht + 4, h.hair)
        c.p(cx + 1, ht + 3, "s" if blink else "a")
        if h.glasses:
            c.r(cx, ht + 2, cx + 3, ht + 2, "j")
        if h.beard:
            c.r(cx, ht + 5, cx + 3, tt + 2, "W")
        hat(c, h, cx, ht, front=False)
        # arm
        s2 = [-1, 0, 1, 0][ph] if ph is not None else 0
        c.r(cx - 1, tt + 1, cx, lt - 2, h.top_s)
        c.r(cx - 1 + s2, lt - 1, cx + s2, lt - 1, h.skin)
        if h.staff:
            sx = cx + 3
            c.r(sx, tt - 4, sx, G, "D"); c.r(sx - 1, tt - 5, sx + 1, tt - 4, "B")
            c.r(cx + 1, tt + 3, cx + 2, tt + 4, h.skin)


def hat(c, h, cx, ht, front):
    if h.hat == "helmet":
        c.r(cx - 4, ht - 1, cx + 3, ht + 2, "q"); c.r(cx - 5, ht + 2, cx + 4, ht + 2, "q")
        c.r(cx - 3, ht - 1, cx + 2, ht - 1, "Q")
    elif h.hat == "hardhat":
        c.r(cx - 4, ht - 1, cx + 3, ht + 1, "y"); c.r(cx - 5, ht + 2, cx + 4, ht + 2, "o")
        if front:
            c.r(cx - 1, ht, cx, ht, "w")
        else:
            c.p(cx + 3, ht, "w")
    elif h.hat == "straw":
        c.r(cx - 6, ht + 1, cx + 5, ht + 1, "T"); c.r(cx - 3, ht - 1, cx + 2, ht, "T")
        c.r(cx - 3, ht, cx + 2, ht, "i")
    elif h.hat == "cap":
        c.r(cx - 3, ht - 1, cx + 2, ht + 1, "i")
        if front:
            c.r(cx - 3, ht + 1, cx + 2, ht + 1, "g")
        else:
            c.r(cx + 2, ht + 1, cx + 4, ht + 1, "g")


PEOPLE = [
    H("elder", "B", "D", "D", "W", hair_style="bald", beard=True, coat=True, staff=True),
    H("child", "i", "g", "k", "y", leg=5, torso=6, head=7, hair_style="pigtails"),
    H("scientist", "w", "l", "#", "D", glasses=True, coat=True),
    H("soldier", "Q", "q", "q", "D", hat="helmet", pack=True),
    H("farmer", "n", "n", "D", "B", hair_style="long", hat="straw", dress=True),
    H("miner", "k", "#", "k", "j", hat="hardhat"),
]


def mirror(im):
    m = im.transpose(Image.FLIP_LEFT_RIGHT)
    out = Image.new("RGBA", im.size, (0, 0, 0, 0)); out.paste(m, (-SCALE, 0))
    return out


def frame(h, view, ph=None, bob=0, blink=False):
    c = C(); human(c, h, view, ph, bob, blink); return c.render()


for h in PEOPLE:
    rows = []
    for view in ("down", "up", "left", "right"):
        v = "right" if view == "left" else view
        fr = [frame(h, v), frame(h, v, bob=1, blink=True)]
        rows.append((f"idle_{view}", [mirror(f) for f in fr] if view == "left" else fr))
    for view in ("down", "up", "left", "right"):
        v = "right" if view == "left" else view
        fr = [frame(h, v, ph=k) for k in range(4)]
        rows.append((f"walk_{view}", [mirror(f) for f in fr] if view == "left" else fr))
    sheet = Image.new("RGBA", (4 * CELL, len(rows) * CELL), (0, 0, 0, 0))
    meta = {"image": f"human_{h.name}.png", "frame_size": [CELL, CELL],
            "origin_feet": [48, 78], "animations": {}}
    for r, (an, frames) in enumerate(rows):
        for ci, im in enumerate(frames):
            sheet.paste(im, (ci * CELL, r * CELL))
        meta["animations"][an] = {"row": r, "frames": len(frames),
                                  "fps": 2 if an.startswith("idle") else 8, "loop": True,
                                  "rects": [[ci * CELL, r * CELL, CELL, CELL] for ci in range(len(frames))]}
    sheet.save(f"{OUT}/human_{h.name}.png")
    json.dump(meta, open(f"{OUT}/human_{h.name}.json", "w"), indent=1)
print("ok")
