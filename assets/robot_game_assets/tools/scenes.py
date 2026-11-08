"""Concept scenes: hopeful post-apocalyptic virtual world.

Environment is drawn procedurally at native resolution (same pixel size as the
characters), characters come from the generated sheets. Output upscaled x3.
Usage: python3 scenes.py <outdir>
"""
import json
import math
import random
import sys
from PIL import Image, ImageDraw

OUT = sys.argv[1] if len(sys.argv) > 1 else "."
UP = 3
W, H = 384, 224


def sh(col, d):
    return tuple(max(0, min(255, c + d)) for c in col[:3])


GR = [(70, 146, 72), (88, 166, 80), (56, 124, 64), (120, 192, 94)]
CON = [(148, 148, 138), (128, 130, 122), (168, 168, 158), (102, 106, 102)]
DIRT = [(150, 116, 82), (126, 96, 66), (170, 136, 98)]
WATER = [(58, 136, 168), (80, 168, 196), (150, 214, 228)]
WALL = [(170, 166, 156), (138, 136, 128), (104, 104, 100), (190, 188, 178)]
IVY = [(44, 112, 58), (72, 152, 68), (110, 186, 84)]
LEAF = [(36, 100, 58), (56, 134, 68), (92, 172, 84), (140, 206, 104)]
TRUNK = [(112, 78, 52), (80, 56, 40)]
RUST = [(176, 96, 54), (134, 68, 42), (92, 52, 36), (206, 132, 80)]
FLOW = [(255, 220, 120), (250, 140, 150), (244, 244, 244), (170, 160, 255)]
CYAN = [(120, 255, 230), (64, 175, 200), (225, 255, 245), (38, 110, 150)]


class Scene:
    def __init__(self, seed, base="grass"):
        self.im = Image.new("RGBA", (W, H), GR[0] + (255,))
        self.rnd = random.Random(seed)
        self.items = []  # (y, fn)
        self.over = Image.new("RGBA", (W, H), (0, 0, 0, 0))
        self.lights = []

    # ------------------------------------------------------------ pixels
    def px(self, x, y, c, a=255, im=None):
        im = im or self.im
        if 0 <= x < W and 0 <= y < H:
            if a >= 255:
                im.putpixel((int(x), int(y)), tuple(c[:3]) + (255,))
            else:
                b = im.getpixel((int(x), int(y)))
                if b[3] == 0:
                    im.putpixel((int(x), int(y)), tuple(c[:3]) + (a,))
                    return
                t = a / 255
                im.putpixel((int(x), int(y)), tuple(int(b[i] * (1 - t) + c[i] * t) for i in range(3)) + (255,))

    def rect(self, x0, y0, x1, y1, c, a=255):
        for y in range(int(y0), int(y1) + 1):
            for x in range(int(x0), int(x1) + 1):
                self.px(x, y, c, a)

    def ellipse(self, cx, cy, rx, ry, c, a=255):
        for y in range(int(cy - ry), int(cy + ry) + 1):
            for x in range(int(cx - rx), int(cx + rx) + 1):
                if ((x - cx) / max(rx, .5)) ** 2 + ((y - cy) / max(ry, .5)) ** 2 <= 1:
                    self.px(x, y, c, a)

    # ------------------------------------------------------------ ground
    def grass(self, x0=0, y0=0, x1=W - 1, y1=H - 1, flowers=0.004):
        r = self.rnd
        self.rect(x0, y0, x1, y1, GR[0])
        for _ in range((x1 - x0) * (y1 - y0) // 14):
            x, y = r.randint(x0, x1), r.randint(y0, y1)
            c = r.choice([GR[1], GR[1], GR[2], GR[3]])
            self.px(x, y, c); self.px(x - 1, y + 1, c); self.px(x + 1, y + 1, c)
        for _ in range(int((x1 - x0) * (y1 - y0) * flowers)):
            x, y = r.randint(x0, x1), r.randint(y0, y1)
            c = r.choice(FLOW)
            self.px(x, y, c); self.px(x, y + 1, GR[2])

    def plaza(self, x0, y0, x1, y1, tile=16):
        r = self.rnd
        for ty in range(y0, y1, tile):
            for tx in range(x0, x1, tile):
                k = r.random()
                if k < 0.12:
                    continue  # missing slab -> grass shows
                base = r.choice(CON[:3])
                self.rect(tx, ty, min(tx + tile - 1, x1), min(ty + tile - 1, y1), base)
                self.rect(tx, ty, min(tx + tile - 1, x1), ty, sh(base, 14))
                self.rect(tx, min(ty + tile - 1, y1), min(tx + tile - 1, x1), min(ty + tile - 1, y1), CON[3])
                self.rect(min(tx + tile - 1, x1), ty, min(tx + tile - 1, x1), min(ty + tile - 1, y1), CON[3])
                for _ in range(r.randint(0, 2)):  # cracks
                    x, y = tx + r.randint(2, tile - 3), ty + r.randint(2, tile - 3)
                    for _ in range(r.randint(4, 10)):
                        self.px(x, y, (88, 90, 88))
                        if r.random() < 0.3:
                            self.px(x + 1, y, IVY[1])
                        x += r.choice((-1, 0, 1)); y += r.choice((0, 1))
                for _ in range(r.randint(0, 5)):  # moss
                    self.px(tx + r.randint(0, tile - 1), ty + tile - 1, IVY[r.randint(0, 2)])
                    self.px(tx + tile - 1, ty + r.randint(0, tile - 1), IVY[r.randint(0, 2)])

    def path(self, pts, w=10):
        r = self.rnd
        for (ax, ay), (bx, by) in zip(pts, pts[1:]):
            n = int(max(abs(bx - ax), abs(by - ay)))
            for t in range(n + 1):
                x = ax + (bx - ax) * t / max(n, 1); y = ay + (by - ay) * t / max(n, 1)
                self.ellipse(x, y, w / 2 + r.random(), w / 3 + r.random(), DIRT[0])
        for _ in range(300):
            (ax, ay), (bx, by) = r.choice(list(zip(pts, pts[1:])))
            t = r.random()
            x = ax + (bx - ax) * t + r.uniform(-w / 2, w / 2)
            y = ay + (by - ay) * t + r.uniform(-w / 3, w / 3)
            self.px(x, y, r.choice(DIRT[1:]))

    def pond(self, cx, cy, rx, ry):
        self.ellipse(cx, cy + 1, rx + 2, ry + 2, (60, 110, 70))
        self.ellipse(cx, cy, rx, ry, WATER[0])
        self.ellipse(cx - 2, cy - 1, rx - 4, ry - 3, WATER[1])
        for i in range(8):
            x = cx - rx + 6 + self.rnd.randint(0, int(rx * 1.5))
            y = cy - ry + 3 + self.rnd.randint(0, int(ry * 1.5))
            if ((x - cx) / rx) ** 2 + ((y - cy) / ry) ** 2 < 0.6:
                self.rect(x, y, x + 3, y, WATER[2])
        for k in range(5):  # lily pads
            a = self.rnd.random() * 6.28
            x, y = cx + math.cos(a) * rx * .6, cy + math.sin(a) * ry * .5
            self.ellipse(x, y, 2, 1, IVY[1]); self.px(x + 1, y - 1, FLOW[1])

    # ------------------------------------------------------------ objects
    def add(self, y, fn, glow=False):
        self.items.append((y, fn, glow))

    def light(self, x, y, rx, ry, col=(120, 255, 230), strength=70):
        self.lights.append((x, y, rx, ry, col, strength))

    def tree(self, x, y, rad=14):
        def f():
            r = random.Random(x * 31 + y)
            self.ellipse(x, y, rad * .9, rad * .35, (30, 70, 40), 110)
            self.rect(x - 2, y - 12, x + 2, y, TRUNK[0])
            self.rect(x + 1, y - 12, x + 2, y, TRUNK[1])
            self.px(x - 3, y, TRUNK[1]); self.px(x + 3, y, TRUNK[1])
            cy = y - 12 - rad * .7
            blobs = [(x + r.randint(-rad // 2, rad // 2), cy + r.randint(-rad // 3, rad // 3),
                      rad * r.uniform(.55, .8)) for _ in range(5)]
            for bx, by, br in blobs:
                self.ellipse(bx, by + 2, br + 1, br * .85 + 1, (24, 74, 46))
            for bx, by, br in blobs:
                self.ellipse(bx, by, br, br * .85, LEAF[1])
            for bx, by, br in blobs:
                self.ellipse(bx - br * .25, by - br * .25, br * .6, br * .5, LEAF[2])
            for bx, by, br in blobs[:3]:
                self.ellipse(bx - br * .4, by - br * .45, br * .25, br * .2, LEAF[3])
            for _ in range(rad * 3):
                bx, by, br = r.choice(blobs)
                self.px(bx + r.uniform(-br, br) * .8, by + r.uniform(-br, br) * .7, r.choice(LEAF[:3]))
            if r.random() < .5:
                for _ in range(3):
                    bx, by, br = r.choice(blobs)
                    self.px(bx + r.randint(-3, 3), by + r.randint(-2, 3), FLOW[r.randint(0, 1)])
        self.add(y, f)

    def bush(self, x, y, rad=6, berries=False):
        def f():
            self.ellipse(x, y, rad, rad * .35, (30, 70, 40), 100)
            self.ellipse(x, y - rad * .5, rad + 1, rad * .75 + 1, (24, 74, 46))
            self.ellipse(x, y - rad * .5, rad, rad * .75, LEAF[1])
            self.ellipse(x - rad * .3, y - rad * .7, rad * .55, rad * .4, LEAF[2])
            if berries:
                r = random.Random(x + y)
                for _ in range(4):
                    self.px(x + r.randint(-rad + 2, rad - 2), y - r.randint(1, rad), FLOW[1])
        self.add(y, f)

    def wall(self, x, y, w, h, windows=True, depth=5):
        def f():
            r = random.Random(x * 7 + y)
            tops = []
            cur = 0
            for i in range(0, w, 3):
                cur = max(0, min(h // 2, cur + r.randint(-3, 3)))
                tops.append(cur)
            self.rect(x, y - 1, x + w, y + 2, (40, 60, 44), 90)  # ground shadow
            for i, t in enumerate(tops):
                cx0 = x + i * 3
                cx1 = min(x + w, cx0 + 2)
                top = y - h + t
                self.rect(cx0, top, cx1, y, WALL[1])
                self.rect(cx0, top - depth, cx1, top - 1, WALL[0])  # top face
                self.rect(cx0, top - depth, cx1, top - depth, WALL[3])
            for yy in range(y - h + 6, y, 7):  # panel seams
                for xx in range(x, x + w):
                    i = (xx - x) // 3
                    if yy > y - h + tops[min(i, len(tops) - 1)] + 1:
                        self.px(xx, yy, WALL[2])
            if windows:
                for wx in range(x + 6, x + w - 8, 18):
                    wy = y - h + 10
                    i = (wx - x) // 3
                    if wy > y - h + tops[min(i, len(tops) - 1)] + 3:
                        self.rect(wx, wy, wx + 7, wy + 10, (52, 58, 62))
                        self.rect(wx, wy + 10, wx + 7, wy + 10, WALL[3])
                        if r.random() < .6:
                            self.rect(wx + 1, wy + 1, wx + 6, wy + 3, IVY[0])
            for _ in range(w // 5):  # ivy curtains
                ix = x + r.randint(0, w)
                i = min((ix - x) // 3, len(tops) - 1)
                iy = y - h + tops[i] - 2
                ln = r.randint(6, h - tops[i])
                for k in range(ln):
                    self.px(ix + (k // 4) % 2, iy + k, IVY[0])
                    if k % 2 == 0:
                        self.px(ix - 1 + r.randint(0, 3), iy + k, r.choice(IVY))
            for i, t in enumerate(tops):  # moss on tops + rebar
                cx0 = x + i * 3
                if r.random() < .5:
                    self.rect(cx0, y - h + t - depth, cx0 + 2, y - h + t - depth + 1, r.choice(IVY[1:]))
                if r.random() < .08:
                    top = y - h + t - depth
                    for k in range(5):
                        self.px(cx0 + 1 + (k // 3), top - k, RUST[1])
            for xx in range(x, x + w + 1):  # grass at base
                if r.random() < .6:
                    self.px(xx, y - r.randint(0, 2), r.choice(GR[1:]))
        self.add(y, f)

    def pillar(self, x, y, h):
        def f():
            r = random.Random(x + y * 3)
            self.ellipse(x + 4, y, 7, 2, (40, 60, 44), 90)
            self.rect(x, y - h, x + 7, y, WALL[1])
            self.rect(x, y - h, x + 1, y, WALL[0])
            self.rect(x + 6, y - h, x + 7, y, WALL[2])
            for i in range(8):
                self.px(x + i, y - h - (r.randint(0, 3)), WALL[0])
            for k in range(h):
                if k % 3 == 0:
                    self.px(x + 2 + int(2 * math.sin(k / 3)), y - h + k, IVY[1])
                    self.px(x + 3 + int(2 * math.sin(k / 3)), y - h + k + 1, IVY[2])
        self.add(y, f)

    def car(self, x, y):
        def f():
            self.ellipse(x + 22, y, 26, 4, (30, 60, 40), 110)
            self.rect(x, y - 14, x + 44, y, RUST[1])
            self.rect(x, y - 14, x + 44, y - 13, RUST[0])
            self.rect(x + 8, y - 22, x + 34, y - 14, RUST[0])
            self.rect(x + 8, y - 22, x + 34, y - 22, RUST[3])
            self.rect(x + 11, y - 20, x + 19, y - 15, (60, 82, 92))
            self.rect(x + 22, y - 20, x + 31, y - 15, (60, 82, 92))
            self.rect(x + 12, y - 20, x + 13, y - 18, (150, 200, 210))
            self.rect(x, y - 6, x + 44, y, RUST[2])
            for xx in (6, 34):
                self.ellipse(x + xx, y - 1, 4, 3, (40, 40, 44))
            r = random.Random(x)
            for _ in range(40):
                self.px(x + r.randint(0, 44), y - r.randint(0, 14), r.choice(RUST))
            for _ in range(30):
                self.px(x + r.randint(-2, 46), y - r.randint(0, 7), r.choice(GR[1:] + IVY))
            for k in range(12):  # vines over roof
                self.px(x + 14 + k * 2, y - 22 + (k % 3), IVY[1])
                self.px(x + 15 + k * 2, y - 21 + (k % 2), IVY[2])
        self.add(y, f)

    def lamp(self, x, y):
        def f():
            for k in range(30):
                bend = 0 if k < 20 else (k - 20) // 2
                self.px(x + bend, y - k, (90, 96, 104)); self.px(x + 1 + bend, y - k, (70, 74, 80))
            self.rect(x + 4, y - 31, x + 9, y - 29, (90, 96, 104))
            for k in range(0, 26, 2):
                self.px(x - 1 + (k // 4) % 3, y - k, IVY[1])
            self.px(x + 7, y - 28, FLOW[0])
        self.add(y, f)

    def rubble(self, x, y, n=6):
        def f():
            r = random.Random(x * 5 + y)
            for _ in range(n):
                rx, ry = x + r.randint(-8, 8), y + r.randint(-3, 3)
                s = r.randint(1, 3)
                self.rect(rx, ry - s, rx + s + 1, ry, WALL[1])
                self.rect(rx, ry - s, rx + s + 1, ry - s, WALL[0])
                if r.random() < .5:
                    self.px(rx + s + 2, ry, GR[3])
        self.add(y, f)

    def spawn(self, x, y, rad=16, beam=True, t=0):
        """Virtual-world spawn pad: stone ring with cyan runes and light beam."""
        def f():
            self.ellipse(x, y + 1, rad + 3, rad * .45 + 2, (40, 60, 50), 120)
            self.ellipse(x, y, rad + 2, rad * .45 + 1, WALL[2])
            self.ellipse(x, y - 1, rad + 1, rad * .45, WALL[0])
            self.ellipse(x, y, rad - 2, rad * .45 - 2, (46, 58, 70))
            for k in range(16):
                a = 2 * math.pi * k / 16 + t
                rx, ry = x + math.cos(a) * (rad - 4), y + math.sin(a) * (rad * .45 - 3)
                self.px(rx, ry, CYAN[0] if k % 2 else CYAN[1])
            self.ellipse(x, y, rad * .35, rad * .15, CYAN[1])
            self.ellipse(x, y, rad * .2, rad * .08, CYAN[2])
            for k in range(6):  # moss creeping on the rim
                a = k * 1.1
                self.px(x + math.cos(a) * (rad + 1), y + math.sin(a) * rad * .45, IVY[1])
        self.add(y, f, glow=True)
        self.light(x, y, rad * 2.2, rad * 1.1)
        if beam:
            def g():
                for yy in range(0, y):
                    fade = yy / max(y, 1)
                    for xx in range(int(x - rad * .5), int(x + rad * .5) + 1):
                        d = abs(xx - x) / (rad * .5)
                        a = int(70 * fade * (1 - d))
                        if a > 4:
                            self.px(xx, yy, CYAN[0], a, im=self.im)
                for k in range(12):
                    r = random.Random(k + int(t * 10))
                    self.px(x + r.randint(-rad // 2, rad // 2), r.randint(y // 3, y - 4), CYAN[2], 200)
            self.add(y + 0.5, g, glow=True)  # beam drawn just after pad, under characters further down

    def butterflies(self, n=5):
        for _ in range(n):
            x, y = self.rnd.randint(10, W - 10), self.rnd.randint(10, H - 30)
            c = self.rnd.choice(FLOW)
            def f(x=x, y=y, c=c):
                self.px(x, y, c); self.px(x + 2, y, c); self.px(x + 1, y + 1, (60, 50, 50))
            self.add(H + 10, f)

    # ------------------------------------------------------------ sprites
    def sprite(self, sheet, anim, idx, fx, fy, shadow=True):
        meta = json.load(open(sheet.replace(".png", ".json")))
        x, yy, w, h = meta["animations"][anim]["rects"][idx]
        img = Image.open(sheet).convert("RGBA").crop((x, yy, x + w, yy + h))
        img = img.resize((w // 2, h // 2), Image.NEAREST)
        ox, oy = meta["origin_feet"][0] // 2, meta["origin_feet"][1] // 2

        def f():
            if shadow:
                bbox = img.getbbox()
                sw = (bbox[2] - bbox[0]) / 2 if bbox else 6
                self.ellipse(fx, fy, max(4, sw * .8), 2, (30, 50, 40), 100)
            self.im.alpha_composite(img, (int(fx - ox), int(fy - oy)))
        self.add(fy + 0.1, f, glow=not shadow and "spirit" in sheet)
        if not shadow and "spirit" in sheet:
            self.light(fx, fy, 10, 4, strength=40)

    # ------------------------------------------------------------ finish
    def render(self, tint=None, rays=True, vignette=True):
        def tintify(layer):
            if not tint:
                return layer
            p = layer.load(); ta = tint[3] / 255
            for y in range(H):
                for x in range(W):
                    q = p[x, y]
                    if q[3]:
                        p[x, y] = tuple(int(q[i] * (1 - ta) + tint[i] * ta) for i in range(3)) + (q[3],)
            return layer
        tintify(self.im)                       # ground
        if tint:                                # light pools on the ground (night/dusk)
            for x, y, rx, ry, col, st in self.lights:
                for yy in range(int(y - ry), int(y + ry) + 1):
                    for xx in range(int(x - rx), int(x + rx) + 1):
                        d = ((xx - x) / rx) ** 2 + ((yy - y) / ry) ** 2
                        if d < 1:
                            a = int(st * (1 - d) * tint[3] / 120)
                            self.px(xx, yy, col, a // 6 * 6)
        main = self.im
        for _, fn, glow in sorted(self.items, key=lambda t: t[0]):
            if glow or not tint:
                fn()
            else:
                layer = Image.new("RGBA", (W, H), (0, 0, 0, 0))
                self.im = layer; fn(); self.im = main
                main.alpha_composite(tintify(layer))
        im = self.im
        if rays:
            ov = Image.new("RGBA", (W, H), (0, 0, 0, 0))
            d = ImageDraw.Draw(ov)
            for k in range(4):
                x0 = 40 + k * 90
                d.polygon([(x0, 0), (x0 + 22, 0), (x0 - 60, H), (x0 - 90, H)], fill=(255, 250, 210, 22))
            im.alpha_composite(ov)
        if vignette:
            ov = Image.new("RGBA", (W, H), (0, 0, 0, 0))
            p = ov.load()
            for y in range(H):
                for x in range(W):
                    d = ((x - W / 2) / (W / 2)) ** 2 + ((y - H / 2) / (H / 2)) ** 2
                    a = int(max(0, d - 0.55) * 90)
                    if a:
                        p[x, y] = (10, 20, 30, min(a, 110) // 8 * 8)
            im.alpha_composite(ov)
        return im.resize((W * UP, H * UP), Image.NEAREST)


HERO = "out2/tv_robot_sheet.png"
NPC = lambda n: f"npc/npc_{n}.png"
SPI = lambda n: f"spirits/spirit_{n}.png"
ANI = lambda n: f"animals/animal_{n}.png"
HUM = lambda n: f"humans/human_{n}.png"


def caption(img, title, sub):
    bar = 70
    out = Image.new("RGB", (img.width, img.height + bar), (22, 26, 38))
    out.paste(img.convert("RGB"), (0, 0))
    d = ImageDraw.Draw(out)
    d.text((18, img.height + 14), title, fill=(225, 255, 245))
    d.text((18, img.height + 38), sub, fill=(150, 175, 185))
    return out


# =================================================================== scenes
def scene_plaza():
    s = Scene(1)
    s.grass(flowers=0.006)
    s.path([(0, 170), (90, 160), (190, 150), (300, 120), (384, 110)], w=14)
    s.plaza(120, 70, 300, 200)
    s.pond(330, 185, 34, 16)
    s.wall(10, 70, 100, 50)
    s.wall(230, 55, 120, 44)
    s.pillar(118, 96, 30); s.pillar(310, 100, 22)
    s.tree(30, 120, 16); s.tree(362, 92, 15); s.tree(80, 214, 14); s.tree(270, 222, 13)
    s.bush(105, 140, 6, True); s.bush(300, 150, 7); s.bush(150, 205, 5, True); s.bush(355, 150, 6)
    s.rubble(140, 120); s.rubble(260, 175, 8)
    s.lamp(215, 96)
    s.spawn(205, 150, 18, beam=True)
    # spirits arriving at the pad
    s.sprite(SPI("farmer"), "appear_down", 3, 198, 150, shadow=False)
    s.sprite(SPI("child"), "idle_left", 0, 216, 156, shadow=False)
    s.sprite(SPI("deer"), "idle_left", 0, 262, 140, shadow=False)
    s.sprite(SPI("hare"), "idle_down", 1, 240, 176, shadow=False)
    s.sprite(SPI("elder"), "walk_left", 1, 168, 186, shadow=False)
    # robots
    s.sprite(HERO, "idle_right", 0, 140, 160)
    s.sprite(NPC("merchant"), "idle_down", 0, 52, 98)
    s.sprite(NPC("gardener"), "walk_right", 1, 112, 196)
    s.sprite(NPC("scrap"), "idle_right", 0, 300, 196)
    s.butterflies(6)
    return caption(s.render(), "SPAWN PLAZA - midday",
                   "Avatars materialise on the spawn pad; the robots who live here see them as passing spirits.")


def scene_forest():
    s = Scene(7)
    s.grass(flowers=0.003)
    s.path([(0, 120), (120, 130), (250, 128), (384, 140)], w=26)  # old road, now dirt
    for x in range(0, W, 24):  # broken asphalt chunks along the road
        if s.rnd.random() < .6:
            y = 124 + s.rnd.randint(-4, 8)
            s.rect(x, y, x + s.rnd.randint(5, 12), y + 3, (78, 82, 86))
            s.rect(x, y, x + 4, y, (98, 102, 106))
    s.car(150, 132)
    s.tree(170, 112, 13)            # tree growing beside/through the car
    s.wall(270, 72, 90, 40, windows=True)
    s.pillar(250, 84, 26)
    for x, y, rr in ((20, 70, 16), (70, 60, 14), (40, 210, 17), (110, 212, 13), (330, 210, 16),
                     (365, 120, 14), (230, 205, 12), (10, 150, 12)):
        s.tree(x, y, rr)
    for x, y in ((90, 100), (205, 165), (300, 170), (60, 160), (350, 160)):
        s.bush(x, y, 6, s.rnd.random() < .5)
    s.rubble(240, 140); s.rubble(320, 98, 5)
    s.spawn(300, 150, 13, beam=False)
    s.sprite(SPI("soldier"), "vanish_down", 2, 300, 150, shadow=False)
    s.sprite(SPI("wolf"), "walk_left", 0, 220, 182, shadow=False)
    s.sprite(SPI("fox"), "pounce_right", 1, 80, 182, shadow=False)
    s.sprite(SPI("bear"), "idle_right", 0, 300, 116, shadow=False)
    s.sprite(SPI("squirrel"), "idle_right", 0, 186, 120, shadow=False)
    s.sprite(NPC("radio"), "idle_down", 0, 120, 150)
    s.sprite(HERO, "walk_right", 1, 60, 132)
    s.butterflies(4)
    return caption(s.render(tint=(255, 130, 50, 70)), "OLD ROAD - golden hour",
                   "A soldier avatar logs off at a small roadside pad; spirit animals roam the reclaimed highway.")


def scene_two_views():
    def build(view):
        s = Scene(21)
        s.grass(flowers=0.006)
        s.plaza(40, 80, 150, 180)
        s.wall(4, 70, 70, 36)
        s.tree(170, 96, 14); s.tree(20, 200, 13)
        s.bush(150, 196, 6, True)
        s.spawn(110, 150, 16, beam=True)
        if view == "avatar":
            s.sprite(HUM("scientist"), "idle_down", 0, 104, 150)
            s.sprite(HUM("miner"), "walk_left", 1, 140, 168)
            s.sprite(ANI("fox"), "idle_left", 0, 70, 186)
            s.sprite(ANI("partridge"), "walk_right", 0, 160, 140)
        else:
            s.sprite(SPI("scientist"), "idle_down", 0, 104, 150, shadow=False)
            s.sprite(SPI("miner"), "walk_left", 1, 140, 168, shadow=False)
            s.sprite(SPI("fox"), "idle_left", 0, 70, 186, shadow=False)
            s.sprite(SPI("partridge"), "walk_right", 0, 160, 140, shadow=False)
        s.sprite(HERO, "idle_right", 0, 56, 150)
        s.sprite(NPC("radio"), "idle_left", 0, 176, 178)
        img = s.render(vignette=False).crop((0, 0, 192 * UP, H * UP))
        return img
    a, b = build("avatar"), build("robot")
    if True:
        # dim / cool the robot view slightly so it reads as a different perception
        ov = Image.new("RGBA", b.size, (20, 40, 70, 40)); b = b.copy(); b.alpha_composite(ov)
    out = Image.new("RGBA", (a.width * 2 + 12, a.height), (22, 26, 38, 255))
    out.alpha_composite(a, (0, 0)); out.alpha_composite(b, (a.width + 12, 0))
    d = ImageDraw.Draw(out)
    for txt, x in (("WHAT THE AVATARS SEE", 14), ("WHAT THE ROBOTS SEE", a.width + 26)):
        d.rectangle((x - 6, 8, x + len(txt) * 6 + 6, 26), fill=(22, 26, 38, 220))
        d.text((x, 12), txt, fill=(225, 255, 245))
    return caption(out, "TWO PERCEPTIONS - same moment, same place",
                   "Visitors are people and animals to each other. To the robots, who live here permanently, they are spirits.")


def scene_night():
    s = Scene(33)
    s.grass(flowers=0.002)
    s.plaza(60, 60, 330, 210)
    s.wall(0, 60, 130, 50); s.wall(250, 50, 134, 46)
    s.pillar(140, 90, 34); s.pillar(236, 88, 30)
    s.tree(20, 150, 16); s.tree(365, 160, 15); s.tree(60, 222, 13); s.tree(330, 222, 14)
    s.bush(120, 200, 6); s.bush(270, 200, 7, True)
    s.spawn(130, 150, 15, beam=True, t=0.4)
    s.spawn(260, 150, 15, beam=True, t=1.2)
    s.spawn(195, 190, 15, beam=False, t=2.0)
    s.sprite(SPI("elder"), "idle_down", 0, 130, 150, shadow=False)
    s.sprite(SPI("child"), "appear_down", 4, 260, 150, shadow=False)
    s.sprite(SPI("farmer"), "walk_down", 2, 160, 176, shadow=False)
    s.sprite(SPI("hedgehog"), "walk_right", 1, 225, 175, shadow=False)
    s.sprite(SPI("partridge"), "flutter_right", 2, 90, 120, shadow=False)
    s.sprite(SPI("deer"), "idle_right", 1, 320, 132, shadow=False)
    s.sprite(HERO, "idle_up", 0, 195, 214)
    s.sprite(NPC("scrap"), "idle_up", 0, 160, 218)
    img = s.render(tint=(14, 24, 60, 120), rays=False)
    # re-light the pads and spirits a bit after the night tint
    return caption(img, "THE GATHERING - night",
                   "When many players log in at once, the plaza fills with lights. The robots come to watch.")


if __name__ != "__main__":
    pass
else:
  for name, fn in (("scene_spawn_plaza", scene_plaza), ("scene_old_road", scene_forest),
                 ("scene_two_perceptions", scene_two_views), ("scene_gathering_night", scene_night)):
    fn().save(f"{OUT}/{name}.png")
    print(name)
