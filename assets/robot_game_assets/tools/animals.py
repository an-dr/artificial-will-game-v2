"""Forest animal sprite sheets in the TV-robot style.

48x48 native grid (32x32 box at (8,8)), 1px outline, scaled x2 -> 96x96 cells,
feet at (48,78) like the hero. Quadrupeds come from one parametric body;
partridge and hedgehog have special bits.
Usage: python3 animals.py <outdir>
"""
import json
import sys
from dataclasses import dataclass, field
from PIL import Image, ImageDraw

OUT = sys.argv[1]
N, BX, SCALE = 48, 8, 2
CELL = N * SCALE
GROUND = 31

P = {
    "a": (11, 11, 15), "#": (50, 62, 79), "j": (33, 31, 52), "w": (244, 244, 244),
    "l": (150, 165, 185), "m": (100, 112, 135), "o": (214, 124, 58), "r": (130, 70, 45),
    "i": (235, 86, 75), "h": (255, 140, 140), "y": (255, 205, 117), "g": (150, 66, 83),
    "B": (150, 95, 55), "D": (95, 60, 40), "T": (225, 190, 140), "K": (58, 38, 30),
    "H": (165, 135, 100), "n": (45, 115, 70), "v": (110, 190, 80),
}


class C:
    def __init__(self):
        self.im = Image.new("RGBA", (N, N), (0, 0, 0, 0))
        self.fx = Image.new("RGBA", (N, N), (0, 0, 0, 0))

    def r(self, x0, y0, x1, y1, col, fx=False):
        im = self.fx if fx else self.im
        for y in range(min(y0, y1), max(y0, y1) + 1):
            for x in range(min(x0, x1), max(x0, x1) + 1):
                if 0 <= x + BX < N and 0 <= y + BX < N:
                    im.putpixel((x + BX, y + BX), P[col] + (255,))

    def p(self, x, y, col, fx=False):
        self.r(x, y, x, y, col, fx)

    def clear(self, x, y):
        if 0 <= x + BX < N and 0 <= y + BX < N:
            self.im.putpixel((x + BX, y + BX), (0, 0, 0, 0))

    def render(self, dx=0):
        src = self.im
        out = Image.new("RGBA", (N, N), (0, 0, 0, 0))
        op = lambda x, y: 0 <= x < N and 0 <= y < N and src.getpixel((x, y))[3]
        for y in range(N):
            for x in range(N):
                if not op(x, y) and any(op(x + a, y + b) for a, b in
                                        ((1, 0), (-1, 0), (0, 1), (0, -1))):
                    out.putpixel((x, y), P["a"] + (255,))
        out.alpha_composite(src)
        out.alpha_composite(self.fx)
        if dx:
            sh = Image.new("RGBA", (N, N), (0, 0, 0, 0)); sh.paste(out, (dx, 0)); out = sh
        return out.resize((CELL, CELL), Image.NEAREST)


@dataclass
class Sp:
    name: str
    bw: int; bh: int; leg: int; lw: int
    hw: int; hh: int; snout: int
    main: str; shade: str; light: str
    ears: str = "point"     # point | round | long | none
    tail: str = "short"     # short | bushy | bushy_down | white | curl | cotton | none
    neck: int = 0           # extra neck height (deer)
    antlers: bool = False
    spikes: bool = False
    nose: str = "a"
    action: str = "leap"
    extra: dict = field(default_factory=dict)


# ---------------------------------------------------------------- side view
def side(c, s, ph=None, dy=0, legs_mode=None, head_dy=0, mouth=False, dx=0,
         blink=False, paw_up=False, tail_flick=0):
    cx = 16 + dx
    by1 = GROUND - s.leg + dy          # body bottom
    by0 = by1 - s.bh + 1               # body top
    bx0 = cx - s.bw // 2
    bx1 = bx0 + s.bw - 1
    # leg offsets (far front, far back, near front, near back) as (dx, lift)
    if legs_mode == "gather":
        L = [(-1, 0), (1, 0), (-1, 0), (1, 0)]
    elif legs_mode == "extend":
        L = [(2, 1), (-2, 1), (2, 0), (-2, 0)]
    elif legs_mode == "tuck":
        L = [(0, 2), (0, 2), (0, 2), (0, 2)]
    elif ph is not None:
        sw = [1, 0, -1, 0][ph]
        lf = [0, 1, 0, 1][ph]
        L = [(-sw, 0), (sw, lf), (sw, lf), (-sw, 0)]
    else:
        L = [(0, 0)] * 4
    if paw_up:
        L[2] = (3, 4)
    fx_, bkx = bx1 - s.lw, bx0 + 1

    def leg(x, lift, col, far):
        top = by1
        bot = GROUND + dy - lift if legs_mode in ("extend", "tuck") else GROUND - lift
        bot = min(bot, GROUND)
        c.r(x, top, x + s.lw - 1, bot, col)
        c.p(x + s.lw - 1 + (0 if far else 0), bot, "K" if s.main != "K" else "a")

    # far legs
    leg(fx_ - 1 + L[0][0], L[0][1], s.shade, True)
    leg(bkx + 1 + L[1][0], L[1][1], s.shade, True)
    # tail behind body
    tail_side(c, s, bx0, by0, by1, tail_flick)
    # body
    c.r(bx0, by0, bx1, by1, s.main)
    c.r(bx0 + 1, by1, bx1 - 1, by1, s.light)
    c.clear(bx0 + BX - BX, by0) if False else None
    for x, y in ((bx0, by0), (bx1, by0), (bx0, by1), (bx1, by1)):
        c.clear(x, y)
    if s.bh >= 4:
        c.r(bx0 + 1, by0, bx1 - 1, by0, s.shade if s.spikes else s.main)
    if s.spikes:
        for x in range(bx0, bx1 + 1):
            c.p(x, by0 - (x % 2), "D")
            if x % 2 == 0:
                c.p(x, by0 + 1, "T")
        c.r(bx0, by0 + 1, bx1 - 2, by1 - 1, "D")
        for x in range(bx0 + 1, bx1 - 2, 2):
            c.p(x, by0 + 2, "T")
    # neck + head
    hx0 = bx1 - 1
    hy1 = by0 + s.hh - 1 - s.neck + head_dy + (1 if s.neck == 0 else 0)
    if s.neck:
        c.r(bx1 - 2, by0 - s.neck, bx1, by0 + 1, s.main)
        hy1 = by0 - s.neck + s.hh - 2 + head_dy
    hy0 = hy1 - s.hh + 1
    c.r(hx0, hy0, hx0 + s.hw - 1, hy1, s.main if not s.spikes else "T")
    if s.snout:
        c.r(hx0 + s.hw, hy1 - 1, hx0 + s.hw + s.snout - 1, hy1, s.main if not s.spikes else "T")
        c.r(hx0 + s.hw, hy1, hx0 + s.hw + s.snout - 1, hy1, s.light)
        c.p(hx0 + s.hw + s.snout - 1, hy1 - 1, s.nose)
        if mouth:
            c.r(hx0 + s.hw, hy1 + 1, hx0 + s.hw + s.snout - 1, hy1 + 1, s.main)
            c.r(hx0 + s.hw, hy1, hx0 + s.hw + s.snout - 1, hy1, "i")
            c.p(hx0 + s.hw + s.snout - 1, hy1, "w")
    ex = hx0 + s.hw - 2
    c.p(ex, hy0 + 1 if s.hh > 2 else hy0, s.shade if blink else "a")
    ears_side(c, s, hx0, hy0)
    if s.antlers:
        ax = hx0 + 1
        c.r(ax, hy0 - 5, ax, hy0 - 1, "T")
        c.r(ax - 2, hy0 - 4, ax - 1, hy0 - 4, "T")
        c.r(ax - 2, hy0 - 7, ax - 2, hy0 - 5, "T")
        c.r(ax + 1, hy0 - 6, ax + 2, hy0 - 6, "T")
        c.p(ax, hy0 - 7, "T")
    # near legs
    leg(fx_ + L[2][0], L[2][1], s.main, False)
    leg(bkx + L[3][0], L[3][1], s.main, False)
    if paw_up:
        px = fx_ + 3 + s.lw
        py = GROUND - 4 + dy
        for k in range(3):
            c.p(px, py - 1 + k, "w")


def ears_side(c, s, hx0, hy0):
    if s.ears == "point":
        c.r(hx0, hy0 - 2, hx0 + 1, hy0 - 1, s.main); c.p(hx0, hy0 - 3, s.main)
        c.p(hx0 + 1, hy0 - 1, s.shade)
    elif s.ears == "round":
        c.r(hx0, hy0 - 1, hx0 + 1, hy0 - 1, s.main)
    elif s.ears == "long":
        c.r(hx0, hy0 - 5, hx0, hy0 - 1, s.main); c.r(hx0 + 1, hy0 - 4, hx0 + 1, hy0 - 1, s.shade)
        c.p(hx0, hy0 - 5, "a")
    elif s.ears == "small":
        c.p(hx0, hy0 - 1, s.main)


def tail_side(c, s, bx0, by0, by1, flick=0):
    t = s.tail
    if t == "bushy":      # fox: long, horizontal-ish, white tip
        c.r(bx0 - 5, by0 + 1, bx0, by0 + 2, s.main)
        c.r(bx0 - 4, by0 + 3, bx0 - 1, by0 + 3, s.main)
        c.r(bx0 - 6, by0 + 1 - flick, bx0 - 5, by0 + 3 - flick, "w")
    elif t == "bushy_down":  # wolf
        c.r(bx0 - 2, by0 + 1, bx0, by0 + 3, s.main)
        c.r(bx0 - 3, by0 + 3, bx0 - 1, by0 + 5, s.main)
        c.r(bx0 - 3, by0 + 5, bx0 - 2, by0 + 6, s.shade)
    elif t == "white":       # deer
        c.r(bx0 - 1, by0 - 1 - flick, bx0, by0 + 1, "w")
    elif t == "curl":        # squirrel: big tail up over the back
        c.r(bx0 - 3, by0 - 5 - flick, bx0 - 1, by1, s.main)
        c.r(bx0 - 2, by0 - 7 - flick, bx0 + 1, by0 - 5 - flick, s.main)
        c.r(bx0 - 3, by0 - 5 - flick, bx0 - 3, by1 - 1, s.shade)
    elif t == "cotton":
        c.r(bx0 - 1, by0, bx0, by0 + 1, "w")
    elif t == "short":
        c.r(bx0 - 1, by0 + 1, bx0, by0 + 2, s.shade)


# ----------------------------------------------------------- front / back
def front(c, s, back=False, ph=None, dy=0, blink=False):
    cx = 16
    fw = max(4, min(s.bh + 4, s.bw // 2 + 2))
    if s.spikes:
        fw = s.bw - 2
    by1 = GROUND - s.leg + dy
    by0 = by1 - s.bh + 1
    x0, x1 = cx - fw // 2, cx - fw // 2 + fw - 1
    lift = [(0, 0), (1, 0), (0, 0), (0, 1)][ph] if ph is not None else (0, 0)
    lx = (x0 + 1, x1 - s.lw)
    # back legs (behind, peeking)
    for i, x in enumerate(lx):
        c.r(x + (1 if i == 0 else -1), by1, x + (1 if i == 0 else -1) + s.lw - 1,
            GROUND - lift[1 - i], s.shade)
    if back:
        hw = s.hw + (1 if s.snout else 0)
        hx0 = cx - hw // 2
        hy1 = by0 + 1 - s.neck
        if s.neck:
            c.r(cx - 1, by0 - s.neck, cx, by0, s.main)
        c.r(hx0, hy1 - s.hh + 1, hx0 + hw - 1, hy1, s.main if not s.spikes else "T")
        ears_front(c, s, hx0, hy1 - s.hh + 1, hw)
        antlers_front(c, s, cx, hy1 - s.hh + 1)
    c.r(x0, by0, x1, by1, s.main)
    for x, y in ((x0, by0), (x1, by0), (x0, by1), (x1, by1)):
        c.clear(x, y)
    if s.spikes:
        for x in range(x0, x1 + 1):
            c.p(x, by0 - (x % 2), "D")
        c.r(x0, by0, x1, by1 - (0 if back else 2), "D")
        for x in range(x0 + 1, x1, 2):
            c.p(x, by0 + 1, "T")
    if back:
        tail_back(c, s, cx, by0, by1)
    # front legs
    for i, x in enumerate(lx):
        c.r(x, by1 - 1, x + s.lw - 1, GROUND - lift[i], s.main)
        c.p(x, GROUND - lift[i], "K" if s.main != "K" else "a")
    if not back:
        c.r(x0 + 1, by0 + 1, x1 - 1, by1 - 1, s.light) if not s.spikes else None
        hw = s.hw + (1 if s.snout else 0)
        hx0 = cx - hw // 2
        hy0 = by0 - s.hh + 2 - s.neck
        if s.neck:
            c.r(cx - 1, hy0 + s.hh - 1, cx, by0 + 1, s.main)
        c.r(hx0, hy0, hx0 + hw - 1, hy0 + s.hh - 1, s.main if not s.spikes else "T")
        if s.snout:
            c.r(cx - 1, hy0 + s.hh - 2, cx, hy0 + s.hh - 1, s.light)
            c.p(cx, hy0 + s.hh - 2, s.nose); c.p(cx - 1, hy0 + s.hh - 2, s.nose)
        ey = hy0 + 1 if s.hh > 2 else hy0
        c.p(hx0, ey, s.shade if blink else "a"); c.p(hx0 + hw - 1, ey, s.shade if blink else "a")
        ears_front(c, s, hx0, hy0, hw)
        antlers_front(c, s, cx, hy0)


def ears_front(c, s, hx0, hy0, hw):
    xs = (hx0, hx0 + hw - 1)
    for i, x in enumerate(xs):
        if s.ears == "point":
            c.r(x, hy0 - 2, x, hy0 - 1, s.main); c.p(x + (1 if i == 0 else -1), hy0 - 1, s.main)
        elif s.ears == "round":
            c.p(x, hy0 - 1, s.main)
        elif s.ears == "long":
            c.r(x, hy0 - 5, x, hy0 - 1, s.main)
        elif s.ears == "small":
            c.p(x, hy0 - 1, s.main)


def antlers_front(c, s, cx, hy0):
    if not s.antlers:
        return
    for sgn in (-1, 1):
        x = cx + (2 if sgn > 0 else -3)
        c.r(x, hy0 - 4, x, hy0 - 1, "T")
        c.r(x + sgn, hy0 - 6, x + sgn, hy0 - 4, "T")
        c.p(x + 2 * sgn, hy0 - 3, "T")
        c.p(x + 2 * sgn, hy0 - 7, "T")


def tail_back(c, s, cx, by0, by1):
    t = s.tail
    if t == "bushy":
        c.r(cx - 1, by0 + 2, cx + 1, by1 + 3, s.main); c.r(cx - 1, by1 + 2, cx + 1, by1 + 3, "w")
    elif t == "bushy_down":
        c.r(cx - 1, by0 + 1, cx, by1 + 2, s.main)
    elif t == "white":
        c.r(cx - 1, by0, cx, by0 + 2, "w")
    elif t == "curl":
        c.r(cx - 2, by0 - 6, cx + 1, by1 - 1, s.main)
        c.r(cx - 1, by0 - 6, cx, by0 - 6, s.shade)
    elif t == "cotton":
        c.r(cx - 1, by0 + 1, cx, by0 + 2, "w")
    elif t == "short":
        c.r(cx - 1, by0 + 1, cx, by0 + 2, s.shade)


# ----------------------------------------------------------- partridge
def bird_side(c, ph=None, dy=0, wing=0, blink=False):
    by = GROUND - 2 + dy
    lift = [(0, 0), (1, 0), (0, 0), (0, 1)][ph] if ph is not None else (0, 0)
    if dy == 0:
        c.r(15, by + 1, 15, GROUND - lift[0], "o"); c.r(17, by + 1, 17, GROUND - lift[1], "o")
    else:
        c.r(15, by + 1, 17, by + 1, "o")
    c.r(11, by - 5, 19, by, "l")          # body
    c.r(11, by - 1, 19, by, "m")
    for x in (13, 15, 17):
        c.r(x, by - 3, x, by - 1, "r")      # chestnut flank bars
    c.r(9, by - 5, 10, by - 3, "B")         # tail
    c.r(19, by - 8, 21, by - 5, "o")        # head (orange face)
    c.r(19, by - 8, 20, by - 8, "B")
    c.p(22, by - 6, "#")                    # beak
    c.p(20, by - 7, "B" if blink else "a")
    c.r(18, by - 4, 19, by - 3, "D")        # chest patch
    if wing == 0:
        c.r(12, by - 4, 16, by - 2, "m")
    elif wing == 1:                          # wings up
        c.r(11, by - 10, 13, by - 5, "m"); c.r(12, by - 12, 13, by - 10, "l")
        c.r(15, by - 9, 17, by - 5, "m")
    else:                                    # wings down
        c.r(11, by, 14, by + 3, "m"); c.r(15, by, 17, by + 2, "m")


def bird_front(c, back=False, ph=None, blink=False):
    by = GROUND - 2
    lift = [(0, 0), (1, 0), (0, 0), (0, 1)][ph] if ph is not None else (0, 0)
    c.r(14, by + 1, 14, GROUND - lift[0], "o"); c.r(17, by + 1, 17, GROUND - lift[1], "o")
    c.r(12, by - 5, 19, by, "l")
    c.r(11, by - 4, 11, by - 1, "m"); c.r(20, by - 4, 20, by - 1, "m")
    if back:
        c.r(14, by - 6, 17, by - 2, "B")
        c.r(14, by - 9, 17, by - 6, "l")
    else:
        c.r(14, by - 4, 17, by - 2, "D")
        c.r(14, by - 9, 17, by - 6, "o")
        c.r(15, by - 6, 16, by - 6, "#")
        c.p(14, by - 8, "B" if blink else "a"); c.p(17, by - 8, "B" if blink else "a")


# ----------------------------------------------------------- species
SPECIES = [
    Sp("deer", 12, 6, 9, 1, 3, 3, 2, "B", "D", "T", ears="point", tail="white",
       neck=4, antlers=True, action="leap"),
    Sp("wolf", 12, 5, 6, 1, 4, 3, 3, "m", "#", "l", ears="point", tail="bushy_down",
       action="bite"),
    Sp("fox", 10, 4, 4, 1, 3, 3, 3, "o", "r", "w", ears="point", tail="bushy",
       action="pounce"),
    Sp("bear", 16, 8, 5, 2, 5, 5, 2, "D", "K", "B", ears="round", tail="short",
       action="swipe"),
    Sp("squirrel", 5, 3, 2, 1, 3, 3, 0, "o", "r", "T", ears="point", tail="curl",
       action="hop"),
    Sp("hedgehog", 8, 3, 1, 1, 3, 2, 2, "T", "D", "T", ears="small", tail="none",
       spikes=True, action="curl"),
    Sp("hare", 7, 4, 3, 1, 3, 3, 1, "H", "B", "T", ears="long", tail="cotton",
       nose="h", action="hop"),
    Sp("partridge", 0, 0, 0, 0, 0, 0, 0, "l", "m", "l", action="flutter"),
]

ACTION_FPS = {"leap": 10, "bite": 10, "pounce": 10, "swipe": 10, "hop": 10,
              "curl": 8, "flutter": 12}
ACTION_LOOP = {"leap": True, "hop": True, "flutter": True}


def hedgehog_ball(c, stage):
    cx, cy = 16, GROUND - 3
    r = 3 if stage < 2 else 4
    for y in range(-r, r + 1):
        for x in range(-r, r + 1):
            if x * x + y * y <= r * r + 1:
                col = "D" if (x + y) % 2 else "T"
                c.p(cx + x, cy + y + (r - 3), col)
    if stage == 1:
        c.r(cx + r - 1, cy + 1, cx + r + 1, cy + 2, "T"); c.p(cx + r + 1, cy + 1, "a")


def action(s, i):
    c = C()
    a = s.action
    if a == "flutter":
        bird_side(c, dy=-[0, 2, 5, 8][i], wing=[1, 2, 1, 2][i])
    elif a == "curl":
        if i == 0:
            side(c, s, dy=0, head_dy=1)
        else:
            hedgehog_ball(c, i)
    elif a in ("leap", "hop"):
        modes = ["gather", "extend", "extend", "gather"]
        dys = [0, -2, -4, -1] if a == "leap" else [0, -3, -4, -1]
        side(c, s, legs_mode=modes[i], dy=dys[i], tail_flick=[0, 1, 2, 0][i])
    elif a == "bite":
        cfg = [dict(dy=1, head_dy=1), dict(dx=3, mouth=True, legs_mode="extend"),
               dict(dx=3, legs_mode="extend"), dict()]
        side(c, s, **cfg[i])
    elif a == "pounce":
        cfg = [dict(dy=1, legs_mode="gather", head_dy=1), dict(dy=-5, dx=1, legs_mode="extend"),
               dict(dy=-3, dx=3, legs_mode="tuck", head_dy=2), dict(dx=4, legs_mode="gather")]
        side(c, s, **cfg[i])
    elif a == "swipe":
        cfg = [dict(dy=1, head_dy=1), dict(paw_up=True, mouth=True, dx=1),
               dict(paw_up=True, mouth=True, dx=2), dict()]
        side(c, s, **cfg[i])
        if i == 2:
            for k in range(4):
                c.p(28 + k // 2, 18 + k * 2, "w", fx=True); c.p(30 + k // 2, 18 + k * 2, "w", fx=True)
    return c.render()


def frame(s, view, ph=None, bob=0, blink=False):
    c = C()
    if s.name == "partridge":
        if view == "right":
            bird_side(c, ph=ph, blink=blink)
        else:
            bird_front(c, back=view == "up", ph=ph, blink=blink)
    else:
        if view == "right":
            side(c, s, ph=ph, dy=0, head_dy=bob, blink=blink)
        else:
            front(c, s, back=view == "up", ph=ph, dy=0, blink=blink)
    return c.render()


def mirror(im):
    m = im.transpose(Image.FLIP_LEFT_RIGHT)
    out = Image.new("RGBA", im.size, (0, 0, 0, 0)); out.paste(m, (-SCALE, 0))
    return out


def build(s):
    rows = []
    for view in ("down", "up", "left", "right"):
        v = "right" if view == "left" else view
        fr = [frame(s, v), frame(s, v, bob=1, blink=True)]
        rows.append((f"idle_{view}", [mirror(f) for f in fr] if view == "left" else fr))
    for view in ("down", "up", "left", "right"):
        v = "right" if view == "left" else view
        fr = [frame(s, v, ph=k) for k in range(4)]
        rows.append((f"walk_{view}", [mirror(f) for f in fr] if view == "left" else fr))
    act = [action(s, i) for i in range(4)]
    rows.append((f"{s.action}_left", [mirror(f) for f in act]))
    rows.append((f"{s.action}_right", act))
    return rows


def fps_of(anim, s):
    k = anim.rsplit("_", 1)[0]
    if k == "idle":
        return 2, True
    if k == "walk":
        return 8, True
    return ACTION_FPS[s.action], ACTION_LOOP.get(s.action, False)


BG = (58, 64, 82)
all_rows = {}
for s in SPECIES:
    rows = build(s)
    all_rows[s.name] = rows
    cols = max(len(f) for _, f in rows)
    sheet = Image.new("RGBA", (cols * CELL, len(rows) * CELL), (0, 0, 0, 0))
    meta = {"image": f"animal_{s.name}.png", "frame_size": [CELL, CELL],
            "origin_feet": [48, 78], "animations": {}}
    for r, (an, frames) in enumerate(rows):
        for ci, im in enumerate(frames):
            sheet.paste(im, (ci * CELL, r * CELL))
        f, lp = fps_of(an, s)
        meta["animations"][an] = {"row": r, "frames": len(frames), "fps": f, "loop": lp,
                                  "rects": [[ci * CELL, r * CELL, CELL, CELL]
                                            for ci in range(len(frames))]}
    sheet.save(f"{OUT}/animal_{s.name}.png")
    json.dump(meta, open(f"{OUT}/animal_{s.name}.json", "w"), indent=1)

    # GIFs: individual (3x) + overview grid (2x)
    def onbg(im, sc):
        bg = Image.new("RGBA", im.size, BG + (255,)); bg.alpha_composite(im)
        return bg.resize((CELL * sc, CELL * sc), Image.NEAREST).convert("RGB")
    for an, frames in rows:
        f, lp = fps_of(an, s)
        imgs = [onbg(x, 3) for x in frames]
        if not lp:  # hold last frame a bit so one-shots read
            imgs += [imgs[-1]] * 4
        imgs[0].save(f"{OUT}/gifs/single/{s.name}_{an}.gif", save_all=True,
                     append_images=imgs[1:], duration=1000 // f, loop=0)
    gc, cs = 5, CELL * 2
    gr = (len(rows) + gc - 1) // gc
    out = []
    for t in range(24):
        cv = Image.new("RGB", (gc * cs, gr * (cs + 14)), BG); d = ImageDraw.Draw(cv)
        for i, (an, frames) in enumerate(rows):
            f, lp = fps_of(an, s)
            k = t * f // 12
            idx = k % len(frames) if lp else min(k % (len(frames) + 3), len(frames) - 1)
            x, y = (i % gc) * cs, (i // gc) * (cs + 14)
            cv.paste(onbg(frames[idx], 2), (x, y))
            d.text((x + 6, y + cs), an, fill=(230, 230, 235))
        out.append(cv)
    out[0].save(f"{OUT}/gifs/{s.name}_all.gif", save_all=True, append_images=out[1:],
                duration=1000 // 12, loop=0)

# lineup with the hero for scale
try:
    hero = Image.open(f"{OUT}/../out2/tv_robot_sheet.png").crop((CELL * 0, CELL * 3, CELL, CELL * 4))
    lu = Image.new("RGBA", (CELL * (len(SPECIES) + 1), CELL), (120, 126, 140, 255))
    lu.alpha_composite(hero, (0, 0))
    for i, s in enumerate(SPECIES):
        lu.alpha_composite(dict(all_rows[s.name])["idle_right"][0], ((i + 1) * CELL, 0))
    lu.save(f"{OUT}/animal_lineup.png")
except FileNotFoundError:
    pass
print("ok")
