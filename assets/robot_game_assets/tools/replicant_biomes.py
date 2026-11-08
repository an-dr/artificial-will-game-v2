"""Two biomes in a NieR Replicant mood (mood only, original designs):
a village of wooden platforms hanging over a gorge, and a bright seaside town.
Imported by biomes.py.
"""
import math
import random

from scenes import (Scene, W, H, GR, CON, WALL, IVY, LEAF, TRUNK, RUST, FLOW, CYAN,
                    HERO, NPC, SPI, caption, sh)

ROCK = [(120, 104, 92), (96, 82, 74), (148, 132, 116), (70, 60, 56)]
PLANK = [(156, 110, 68), (124, 86, 54), (184, 140, 92), (92, 64, 42)]
THATCH = [(214, 180, 104), (186, 150, 80), (236, 206, 136)]
CLOTH = [(206, 74, 70), (70, 120, 190), (236, 210, 120)]
SAND = [(236, 214, 164), (218, 192, 140), (246, 230, 190)]
SEA = [(44, 160, 180), (70, 190, 200), (150, 226, 226), (240, 252, 250), (30, 120, 150)]
WHITE = [(236, 232, 222), (210, 204, 194), (250, 248, 242), (180, 174, 166)]
BLUEP = [(52, 104, 168), (36, 76, 130)]
PINK = [(232, 96, 150), (250, 150, 190)]


# ======================================================== CLIFF VILLAGE
def cliff_village():
    s = Scene(505)
    s.grass(flowers=0.005)
    r = s.rnd
    # gorge edges (wavy)
    left = [150 + int(8 * math.sin(y / 17) + 4 * math.sin(y / 5)) for y in range(H)]
    right = [236 + int(8 * math.sin(y / 13 + 2) + 3 * math.sin(y / 7)) for y in range(H)]
    for y in range(H):
        for x in range(left[y], right[y]):
            depth = min(abs(x - left[y]), abs(right[y] - x))
            t = min(1, depth / 30)
            c = tuple(int(ROCK[1][i] * (1 - t) + (16, 18, 26)[i] * t) for i in range(3))
            s.px(x, y, c)
        for k in range(10):  # rock faces with strata
            s.px(left[y] + k, y, ROCK[0] if (y + k * 3) % 9 > 1 else ROCK[3])
            s.px(right[y] - 1 - k, y, ROCK[1] if (y + k * 3) % 9 > 1 else ROCK[3])
        s.px(left[y] - 1, y, ROCK[2]); s.px(right[y], y, ROCK[2])
    for y in range(H):  # river far below
        x = 193 + int(4 * math.sin(y / 20))
        s.rect(x - 2, y, x + 2, y, (40, 80, 110)); s.px(x, y, (90, 150, 180))
    for k in range(9):  # mist bands in the gorge
        y = 20 + k * 24 + r.randint(-5, 5)
        for x in range(left[min(y, H - 1)], right[min(y, H - 1)]):
            s.px(x, y, (200, 215, 225), 50); s.px(x, y + 1, (200, 215, 225), 30)
    # waterfall from the left cliff
    for y in range(0, 70):
        for x in range(left[y] - 2, left[y] + 8):
            s.px(x, y, (220, 240, 250) if (x + y) % 4 else (150, 200, 225))
    for k in range(40):
        s.px(left[70] + r.randint(-4, 14), 70 + r.randint(-6, 6), (240, 250, 255), 150)

    def deck(x, y, w, h, side):
        def f():
            for k in range(0, w, 8):  # struts going down into the gorge
                for t in range(18):
                    s.px(x + k + (t // 3 if side == "L" else -t // 3), y + t, PLANK[3])
            s.rect(x, y - h, x + w, y, PLANK[0])
            for yy in range(y - h, y + 1, 3):
                s.rect(x, yy, x + w, yy, PLANK[1])
            s.rect(x, y - h, x + w, y - h, PLANK[2])
            s.rect(x, y + 1, x + w, y + 2, PLANK[3])
            for xx in range(x, x + w + 1, 6):
                s.rect(xx, y - 4, xx, y, PLANK[3])
            s.rect(x, y - 4, x + w, y - 4, PLANK[1])
        s.add(y - h, f)

    def bridge(x0, x1, y):
        def f():
            n = x1 - x0
            for t in range(n):
                sag = 5 * math.sin(math.pi * t / n)
                yy = y + sag
                if t % 3 != 2:
                    s.rect(x0 + t, yy - 3, x0 + t, yy + 3, PLANK[0] if t % 2 else PLANK[2])
                s.px(x0 + t, yy - 6 - sag * .3, (200, 180, 140))
                s.px(x0 + t, yy + 5 - sag * .3, (170, 150, 112))
                if t % 9 == 0:
                    s.rect(x0 + t, yy - 6, x0 + t, yy - 3, (170, 150, 112))
        s.add(y - 3, f)

    def hut(x, y, rad=14, cloth=0):
        def f():
            s.ellipse(x, y + 1, rad + 2, rad * .4, (40, 60, 44), 110)
            s.rect(x - rad + 2, y - 14, x + rad - 2, y, PLANK[1])
            for xx in range(x - rad + 2, x + rad - 1, 3):
                s.rect(xx, y - 14, xx, y, PLANK[3])
            s.rect(x - 3, y - 9, x + 3, y, (50, 36, 30))
            apex = (x, y - 14 - rad * 1.2)
            for yy in range(int(apex[1]), y - 13):
                tt = (yy - apex[1]) / (y - 14 - apex[1])
                half = (rad + 2) * tt
                for xx in range(int(x - half), int(x + half) + 1):
                    c = THATCH[0] if (xx + yy) % 5 else THATCH[1]
                    if xx < x - half * .4:
                        c = THATCH[2] if (xx + yy) % 5 else THATCH[0]
                    s.px(xx, yy, c)
            s.rect(x - rad - 2, y - 14, x + rad + 2, y - 13, THATCH[1])
            for k in range(14):  # pennant pole + cloth streamer
                s.px(x, apex[1] - k, PLANK[3])
            for k in range(10):
                s.px(x + 1 + k, apex[1] - 13 + int(1.5 * math.sin(k / 2)), CLOTH[cloth])
                s.px(x + 1 + k, apex[1] - 12 + int(1.5 * math.sin(k / 2)), CLOTH[cloth])
        s.add(y, f)

    def windmill(x, y):
        def f():
            s.rect(x - 4, y - 30, x + 4, y, PLANK[1]); s.rect(x - 4, y - 30, x - 2, y, PLANK[2])
            cx, cy = x, y - 30
            for a in (20, 110, 200, 290):
                for t in range(18):
                    px_ = cx + math.cos(math.radians(a)) * t; py_ = cy + math.sin(math.radians(a)) * t
                    s.px(px_, py_, PLANK[3])
                    if t > 5:
                        s.px(px_ + math.cos(math.radians(a + 90)) * 2, py_ + math.sin(math.radians(a + 90)) * 2,
                             (230, 222, 200))
            s.ellipse(cx, cy, 2, 2, PLANK[3])
        s.add(y, f)

    deck(108, 70, 48, 26, "L"); deck(230, 66, 50, 24, "R")
    deck(110, 170, 46, 30, "L"); deck(232, 176, 44, 26, "R")
    bridge(156, 232, 58)
    bridge(156, 234, 156)
    hut(50, 70, 15, 0); hut(90, 140, 13, 1); hut(320, 60, 15, 2); hut(300, 140, 14, 0)
    windmill(360, 120)
    for gx in range(20, 90, 4):   # little terraced field
        for gy in range(178, 214, 6):
            s.px(gx, gy, LEAF[2]); s.px(gx + 1, gy - 1, LEAF[1])
    s.tree(20, 120, 13); s.tree(366, 210, 14); s.tree(280, 216, 12)
    s.bush(130, 110, 5, True); s.bush(262, 110, 6)
    s.spawn(330, 190, 13, beam=True)
    s.sprite(SPI("deer"), "walk_right", 1, 186, 158, shadow=False)
    s.sprite(SPI("child"), "walk_left", 2, 214, 60, shadow=False)
    s.sprite(SPI("farmer"), "idle_down", 0, 330, 190, shadow=False)
    s.sprite(SPI("partridge"), "flutter_left", 2, 196, 110, shadow=False)
    s.sprite(HERO, "idle_right", 0, 132, 160)
    s.sprite(NPC("gardener"), "walk_left", 1, 60, 200)
    s.sprite(NPC("radio"), "idle_down", 0, 254, 60)
    return caption(s.render(tint=(200, 220, 255, 22)),
                   "THE GORGE VILLAGE - morning",
                   "Huts and decks cling to both sides of a misty gorge, joined by rope bridges. A waterfall drowns out everything else.")


# ============================================================== SEAFRONT
def seafront():
    s = Scene(606)
    r = s.rnd
    s.grass(0, 0, W, 40, flowers=0.01)
    # promenade
    for ty in range(40, 150, 8):
        off = 4 if (ty // 8) % 2 else 0
        for tx in range(-4 + off, W, 8):
            c = r.choice(WHITE[:3])
            s.rect(tx, ty, tx + 6, ty + 6, c); s.rect(tx, ty + 6, tx + 6, ty + 6, WHITE[3])
            if r.random() < .06:
                s.px(tx + 3, ty + 3, IVY[1])
    # beach + sea
    shore = [168 + int(5 * math.sin(x / 23) + 3 * math.sin(x / 9)) for x in range(W)]
    for x in range(W):
        s.rect(x, 150, x, shore[x], SAND[0])
        for y in range(shore[x], H):
            d = y - shore[x]
            s.px(x, y, SEA[2] if d < 4 else SEA[1] if d < 18 else SEA[0])
        s.px(x, shore[x], SEA[3]); s.px(x, shore[x] + 1, SEA[3] if x % 3 else SEA[2])
    s.rect(0, 148, W, 150, WHITE[3])
    for _ in range(120):
        x = r.randint(0, W - 1)
        s.px(x, r.randint(152, shore[x] - 1), SAND[r.randint(1, 2)])
    for _ in range(90):
        x, y = r.randint(0, W), r.randint(185, H)
        s.rect(x, y, x + r.randint(2, 6), y, SEA[2] if y < 200 else SEA[1])
    for x, y in ((60, 162), (150, 160), (300, 164)):  # shells / starfish
        s.px(x, y, (240, 150, 120)); s.px(x + 1, y, (250, 200, 170))

    def house(x, y, w, h, dome=False, broken=False):
        def f():
            rr = random.Random(x)
            s.rect(x - 1, y + 1, x + w + 1, y + 3, (150, 150, 140), 120)
            s.rect(x, y - h, x + w, y - 10, WHITE[2])   # roof (flat, top view)
            s.rect(x, y - h, x + w, y - h, WHITE[1])
            s.rect(x, y - 10, x + w, y, WHITE[0])      # front wall
            s.rect(x + w - 2, y - 10, x + w, y, WHITE[1])
            s.rect(x + w // 2 - 2, y - 7, x + w // 2 + 2, y, BLUEP[0])
            for wx in (x + 3, x + w - 7):
                s.rect(wx, y - 8, wx + 3, y - 5, BLUEP[1]); s.rect(wx - 1, y - 8, wx - 1, y - 5, BLUEP[0])
                s.rect(wx + 4, y - 8, wx + 4, y - 5, BLUEP[0])
            if dome:
                s.ellipse(x + w / 2, y - h + 8, w * .3, 6, BLUEP[0])
                s.ellipse(x + w / 2 - 2, y - h + 6, w * .14, 3, (110, 160, 220))
            for _ in range(6):  # bougainvillea
                fx, fy = x + rr.randint(0, w), y - rr.randint(4, 11)
                s.px(fx, fy, PINK[rr.randint(0, 1)]); s.px(fx + 1, fy, PINK[0]); s.px(fx, fy + 1, IVY[1])
            if broken:
                for k in range(w // 2):
                    s.rect(x + w - k, y - h, x + w - k, y - h + rr.randint(4, h - 12), (120, 112, 104))
                for k in range(w // 2):
                    s.rect(x + w - k, y - h, x + w - k, y - h + rr.randint(2, 10), GR[1])
                s.rubble(x + w + 4, y + 4, 5)
            for k in range(rr.randint(2, 6)):
                s.px(x + rr.randint(0, w), y - h + rr.randint(0, 3), IVY[rr.randint(0, 2)])
        s.add(y, f)

    def lighthouse(x, y):
        def f():
            s.ellipse(x, y + 2, 20, 8, ROCK[1]); s.ellipse(x - 3, y, 14, 6, ROCK[0])
            for yy in range(y - 70, y):
                tt = (y - yy) / 70
                half = int(9 - 3 * tt)
                band = ((y - yy) // 12) % 2
                for xx in range(x - half, x + half + 1):
                    c = (206, 70, 64) if band else WHITE[2]
                    if xx > x + half - 2:
                        c = sh(c, -30)
                    s.px(xx, yy, c)
            s.rect(x - 8, y - 76, x + 8, y - 70, (60, 60, 70))
            s.rect(x - 4, y - 75, x + 4, y - 71, (255, 230, 140))
            for xx in range(x - 9, x + 10, 2):
                s.px(xx, y - 70, (90, 90, 100))
            for yy in range(y - 84, y - 76):
                half = (yy - (y - 84)) // 1
                s.rect(x - half, yy, x + half, yy, (206, 70, 64))
            for k in range(16):  # light beam, faint
                s.px(x + 8 + k * 2, y - 74 - k // 2, (255, 240, 180), 100)
                s.px(x + 8 + k * 2, y - 73 - k // 2, (255, 240, 180), 70)
        s.add(y, f)

    def pier(x, y0, y1):
        def f():
            for yy in range(y0, y1, 3):
                s.rect(x, yy, x + 16, yy + 1, PLANK[0] if (yy // 3) % 2 else PLANK[2])
                s.rect(x, yy + 2, x + 16, yy + 2, PLANK[3])
            for yy in range(y0 + 4, y1, 12):
                s.rect(x - 1, yy, x, yy + 3, PLANK[3]); s.rect(x + 16, yy, x + 17, yy + 3, PLANK[3])
            for xx in range(x - 2, x + 19, 2):
                s.px(xx, y1 + 1, SEA[3])
        s.add(y0, f)

    def boat(x, y):
        def f():
            s.ellipse(x, y, 14, 4, PLANK[1]); s.ellipse(x, y - 1, 12, 3, PLANK[2])
            s.rect(x - 9, y - 1, x + 9, y, PLANK[3])
            for k in range(20):
                s.px(x, y - k, PLANK[3])
            for k in range(14):
                s.rect(x + 1, y - 18 + k, x + 1 + (14 - k) // 2, y - 18 + k, WHITE[2])
            for xx in range(x - 16, x + 17, 2):
                s.px(xx, y + 4, SEA[3])
        s.add(y, f)

    house(10, 40, 46, 34, dome=True); house(70, 36, 38, 30)
    house(124, 42, 50, 36, broken=True); house(196, 38, 40, 30, dome=True)
    house(254, 40, 44, 34)
    s.tree(318, 40, 12); s.tree(14, 120, 11)
    for x, y in ((40, 120), (110, 116), (176, 120), (236, 112)):  # flower pots on the promenade
        def f(x=x, y=y):
            s.rect(x - 3, y - 4, x + 3, y, (196, 110, 70)); s.rect(x - 3, y - 4, x + 3, y - 4, (220, 140, 90))
            s.ellipse(x, y - 7, 4, 3, LEAF[1]); s.px(x - 1, y - 8, PINK[0]); s.px(x + 2, y - 7, FLOW[0])
        s.add(y, f)
    lighthouse(352, 120)
    pier(250, 150, 214)
    boat(210, 204)
    s.spawn(258, 200, 10, beam=True)
    s.sprite(SPI("child"), "walk_left", 1, 120, 168, shadow=False)
    s.sprite(SPI("elder"), "idle_down", 1, 258, 200, shadow=False)
    s.sprite(SPI("partridge"), "flutter_right", 1, 300, 160, shadow=False)
    s.sprite(SPI("hare"), "idle_right", 1, 70, 158, shadow=False)
    s.sprite(SPI("farmer"), "walk_right", 2, 200, 130, shadow=False)
    s.sprite(HERO, "walk_right", 1, 150, 110)
    s.sprite(NPC("merchant"), "idle_down", 0, 290, 84)
    s.sprite(NPC("scrap"), "idle_left", 0, 64, 92)
    return caption(s.render(),
                   "THE WHITE HARBOUR - afternoon",
                   "Whitewashed houses, bougainvillea, an old lighthouse. A pad at the end of the pier brings visitors in by the sea.")
