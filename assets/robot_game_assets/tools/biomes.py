"""Four more biomes for the virtual post-apocalyptic world.

Moods: sand-swallowed city, drowned city, rusting works, faded amusement park.
Reuses the Scene engine from scenes.py. Usage: python3 biomes.py <outdir>
"""
import math
import random
import sys

sys.argv = [sys.argv[0], sys.argv[1] if len(sys.argv) > 1 else "."]
from scenes import (Scene, W, H, UP, GR, CON, WALL, IVY, LEAF, TRUNK, RUST, FLOW, CYAN,
                    HERO, NPC, SPI, caption, sh)

OUT = sys.argv[1]

SAND = [(224, 192, 132), (206, 170, 110), (240, 214, 160), (176, 138, 88)]
DRYG = [(170, 160, 90), (140, 130, 70)]
WAT = [(40, 104, 132), (58, 134, 162), (110, 190, 210), (210, 240, 245), (30, 80, 104)]
METAL = [(112, 120, 128), (92, 98, 106), (140, 148, 156), (70, 74, 82)]
PAVE = [(176, 150, 132), (150, 126, 112), (196, 172, 152)]
STRIPE = [(232, 82, 76), (244, 236, 220), (82, 150, 210), (250, 200, 90)]
WOOD = [(150, 104, 64), (118, 80, 50), (178, 132, 84)]


# ================================================================ helpers
def mound(s, x, y, rx, ry):
    s.ellipse(x, y, rx, ry, SAND[1])
    s.ellipse(x - rx * .15, y - ry * .3, rx * .8, ry * .7, SAND[0])
    s.ellipse(x - rx * .35, y - ry * .5, rx * .4, ry * .3, SAND[2])


def tower(s, x, y, w, h, bury=18, broken=True, col=WALL, windows=True, sunk=False):
    """Generic ruined building block; base buried in sand or sunk in water."""
    def f():
        r = random.Random(x * 13 + y)
        top = y - h
        tops = [r.randint(0, 6) if broken else 0 for _ in range(0, w, 3)]
        for i, t in enumerate(tops):
            x0 = x + i * 3
            s.rect(x0, top + t, min(x0 + 2, x + w), y, col[1])
            s.rect(x0, top + t - 4, min(x0 + 2, x + w), top + t - 1, col[0])
        s.rect(x + w - 3, top + 4, x + w, y, col[2])
        if windows:
            for wy in range(top + 8, y - 4, 10):
                for wx in range(x + 3, x + w - 5, 8):
                    s.rect(wx, wy, wx + 4, wy + 5, (48, 54, 60))
                    if r.random() < .3:
                        s.rect(wx + 1, wy + 1, wx + 3, wy + 2, IVY[0])
        if sunk:
            for yy in range(y - 6, y + 1):
                for xx in range(x - 2, x + w + 3):
                    s.px(xx, yy, WAT[1], 150)
            for xx in range(x - 4, x + w + 5, 2):
                s.px(xx, y + 1 + (xx % 4 == 0), WAT[3])
        else:
            mound(s, x + w / 2, y, w * .75, bury * .5)
    s.add(y + (8 if not sunk else 0), f)


# ================================================================ DESERT
def desert():
    s = Scene(101)
    s.rect(0, 0, W, H, SAND[0])
    r = s.rnd
    for k in range(9):  # dune ridges
        y0 = 20 + k * 24 + r.randint(-6, 6)
        ph = r.random() * 6
        for x in range(W):
            y = y0 + 5 * math.sin(x / 40 + ph)
            s.px(x, y, SAND[2]); s.px(x, y + 1, SAND[2])
            for d in range(2, 7):
                s.px(x, y + d, SAND[1], 120 - d * 15)
    for _ in range(500):  # ripple marks
        x, y = r.randint(0, W), r.randint(0, H)
        s.rect(x, y, x + r.randint(2, 4), y, SAND[1])
    for _ in range(120):
        s.px(r.randint(0, W), r.randint(0, H), SAND[3])
    # oasis around a spawn pad
    s.ellipse(255, 160, 46, 22, (150, 160, 90))
    s.grass(215, 145, 295, 178, flowers=0.002)
    for yy in range(138, 184):  # soften grass patch into sand
        for xx in range(205, 306):
            d = ((xx - 255) / 50) ** 2 + ((yy - 160) / 24) ** 2
            if d > 0.75:
                s.px(xx, yy, SAND[0], int(min(1, (d - .75) * 4) * 255))
    s.pond(272, 170, 16, 7)
    s.spawn(240, 158, 14, beam=True)
    # half-buried city
    tower(s, 20, 96, 48, 80); tower(s, 86, 70, 34, 56, broken=True)
    tower(s, 310, 82, 56, 64); tower(s, 150, 52, 26, 40)

    def giant_head(x=120, y=200):  # huge buried machine head - landmark
        def f():
            s.ellipse(x, y - 12, 34, 26, RUST[1])
            s.ellipse(x - 6, y - 20, 26, 18, RUST[0])
            s.ellipse(x - 14, y - 26, 10, 6, RUST[3])
            s.ellipse(x + 8, y - 16, 9, 8, (40, 30, 30))   # eye socket
            s.ellipse(x + 8, y - 16, 4, 3, (90, 60, 50))
            for k in range(6):
                s.rect(x - 30 + k * 10, y - 4, x - 26 + k * 10, y, RUST[2])
            rr = random.Random(5)
            for _ in range(60):
                s.px(x + rr.randint(-32, 32), y - rr.randint(0, 34), rr.choice(RUST))
            s.rect(x - 16, y - 38, x - 14, y - 30, RUST[2])   # antenna stub
            mound(s, x, y + 2, 46, 12)
            for k in range(5):
                s.px(x - 20 + k * 3, y - 34 + (k % 2), DRYG[0])
        s.add(y + 4, f)
    giant_head()

    def palm(x, y, dead=False):
        def f():
            s.ellipse(x, y, 8, 2, (150, 120, 80), 120)
            for k in range(22):
                bx = x + int(3 * math.sin(k / 8))
                s.px(bx, y - k, TRUNK[0]); s.px(bx + 1, y - k, TRUNK[1])
            tx, ty = x + int(3 * math.sin(22 / 8)), y - 22
            fronds = DRYG if dead else [LEAF[1], LEAF[2]]
            for a in range(0, 360, 50):
                for t in range(10):
                    fx = tx + math.cos(math.radians(a)) * t
                    fy = ty + math.sin(math.radians(a)) * t * .5 + t * t * .05
                    s.px(fx, fy, fronds[t % 2])
        s.add(y, f)
    palm(222, 150); palm(292, 154); palm(205, 176, dead=True); palm(350, 190, dead=True)
    for x, y in ((40, 150), (170, 120), (330, 120), (60, 205)):
        def f(x=x, y=y):
            for a in range(0, 360, 40):
                s.px(x + math.cos(math.radians(a)) * 3, y - 2 + math.sin(math.radians(a)) * 2, DRYG[1])
            s.px(x, y - 2, DRYG[0])
        s.add(y, f)
    s.rubble(190, 100, 5); s.rubble(60, 120, 4)
    s.sprite(SPI("elder"), "idle_down", 0, 240, 158, shadow=False)
    s.sprite(SPI("fox"), "walk_left", 2, 300, 196, shadow=False)
    s.sprite(SPI("hare"), "hop_right", 1, 180, 168, shadow=False)
    s.sprite(SPI("soldier"), "walk_right", 1, 198, 144, shadow=False)
    s.sprite(HERO, "walk_right", 1, 150, 150)
    s.sprite(NPC("scrap"), "walk_right", 2, 52, 176)
    s.sprite(NPC("radio"), "idle_left", 0, 86, 118)
    return caption(s.render(tint=(255, 200, 120, 26)),
                   "THE DUNE CITY - noon",
                   "Sand swallowed the towers to their third floor. A giant machine head is a landmark; the oasis pad is the only green for miles.")


# ================================================================ FLOODED
def flooded():
    s = Scene(202)
    s.rect(0, 0, W, H, WAT[1])
    r = s.rnd
    for _ in range(900):
        x, y = r.randint(0, W), r.randint(0, H)
        s.rect(x, y, x + r.randint(1, 5), y, r.choice([WAT[0], WAT[2], WAT[0]]))
    for _ in range(40):  # submerged street lines / old cars under water
        x, y = r.randint(0, W), r.randint(0, H)
        s.rect(x, y, x + r.randint(8, 20), y + r.randint(3, 6), WAT[4], 90)

    def roof(x, y, w, h, garden=False, rail=True):
        def f():
            rr = random.Random(x + y * 7)
            s.rect(x - 2, y - h - 2, x + w + 2, y + 6, WAT[4], 110)       # submerged walls
            s.rect(x, y - h, x + w, y, CON[0])
            s.rect(x, y - h, x + w, y - h + 1, CON[2])
            s.rect(x, y, x + w, y + 4, WALL[2])                          # wall face above water
            for xx in range(x - 3, x + w + 4, 2):
                s.px(xx, y + 5 + (xx % 3 == 0), WAT[3])
            for _ in range(w * h // 30):
                s.px(x + rr.randint(1, w - 1), y - rr.randint(1, h - 1), rr.choice(IVY + [CON[1]]))
            if rail:
                for xx in range(x, x + w + 1, 4):
                    s.rect(xx, y - h - 4, xx, y - h, (90, 96, 104))
                s.rect(x, y - h - 4, x + w, y - h - 4, (110, 116, 124))
            if garden:
                s.rect(x + 4, y - h + 4, x + w - 4, y - 4, (110, 84, 60))
                for gx in range(x + 6, x + w - 5, 3):
                    for gy in range(y - h + 6, y - 5, 4):
                        s.px(gx, gy, LEAF[2]); s.px(gx + 1, gy - 1, LEAF[1])
                        if rr.random() < .3:
                            s.px(gx, gy - 1, FLOW[rr.randint(0, 3)])
            else:
                s.rect(x + w - 14, y - h + 4, x + w - 5, y - h + 10, METAL[0])   # AC unit
                s.rect(x + w - 14, y - h + 4, x + w - 5, y - h + 4, METAL[2])
                s.ellipse(x + w - 9, y - h + 7, 2, 2, METAL[3])
        s.add(y - h, f)

    def planks(x0, y0, x1, y1):
        def f():
            n = max(abs(x1 - x0), abs(y1 - y0))
            for t in range(0, n + 1):
                x = x0 + (x1 - x0) * t / n; y = y0 + (y1 - y0) * t / n
                s.rect(x, y - 2, x + 1, y + 2, WOOD[0] if (t // 2) % 2 else WOOD[2])
                s.px(x, y + 3, WOOD[1])
        s.add(max(y0, y1) - 1, f)

    def reeds(x, y):
        def f():
            rr = random.Random(x * y)
            for _ in range(7):
                rx = x + rr.randint(-5, 5); hgt = rr.randint(5, 10)
                for k in range(hgt):
                    s.px(rx + (k > hgt - 3), y - k, IVY[1] if k < hgt - 2 else (150, 110, 70))
            for xx in range(x - 6, x + 7, 2):
                s.px(xx, y + 1, WAT[3])
        s.add(y, f)

    roof(20, 120, 90, 50, rail=True)
    roof(150, 190, 80, 46, garden=True)
    roof(270, 110, 96, 44)
    tower(s, 140, 70, 40, 70, sunk=True)
    tower(s, 300, 46, 30, 46, sunk=True)
    planks(110, 100, 150, 160)
    planks(230, 170, 280, 100)
    s.tree(60, 96, 13)          # tree growing on a roof
    s.tree(196, 166, 11)
    for x, y in ((30, 200), (120, 210), (250, 210), (360, 190), (240, 70), (20, 40)):
        reeds(x, y)
    for k in range(10):
        x, y = r.randint(0, W), r.randint(0, H)
        s.ellipse(x, y, 2, 1, IVY[1]); s.px(x + 1, y - 1, FLOW[r.randint(0, 2)])

    def lamp_in_water(x, y):
        def f():
            for k in range(26):
                s.px(x, y - k, (90, 96, 104))
            s.rect(x - 1, y - 28, x + 4, y - 26, (110, 116, 124))
            for xx in range(x - 4, x + 5, 2):
                s.px(xx, y + 1, WAT[3])
        s.add(y, f)
    lamp_in_water(246, 140); lamp_in_water(120, 60)
    s.spawn(300, 92, 12, beam=True)   # pad on the right rooftop
    s.sprite(SPI("child"), "appear_down", 5, 300, 92, shadow=False)
    s.sprite(SPI("deer"), "walk_left", 1, 250, 150, shadow=False)    # spirits walk on water
    s.sprite(SPI("farmer"), "walk_down", 1, 110, 170, shadow=False)
    s.sprite(SPI("partridge"), "flutter_right", 2, 200, 120, shadow=False)
    s.sprite(HERO, "idle_right", 0, 70, 116)
    s.sprite(NPC("gardener"), "idle_down", 1, 176, 176)
    return caption(s.render(tint=(70, 120, 150, 40)),
                   "THE DROWNED DISTRICT - overcast",
                   "Streets lie a storey under water. Robots cross on plank bridges between rooftop gardens; spirits simply walk over the water.")


# ================================================================ FACTORY
def factory():
    s = Scene(303)
    r = s.rnd
    for ty in range(0, H, 16):
        for tx in range(0, W, 16):
            base = r.choice(METAL[:3])
            s.rect(tx, ty, tx + 15, ty + 15, base)
            s.rect(tx, ty + 15, tx + 15, ty + 15, METAL[3]); s.rect(tx + 15, ty, tx + 15, ty + 15, METAL[3])
            for cx, cy in ((2, 2), (13, 2), (2, 13), (13, 13)):
                s.px(tx + cx, ty + cy, METAL[2] if base != METAL[2] else METAL[0])
            for k in range(0, 12, 4):
                s.px(tx + 4 + k, ty + 7, METAL[3]); s.px(tx + 5 + k, ty + 6, METAL[3])
            if r.random() < .3:
                s.ellipse(tx + r.randint(3, 12), ty + r.randint(3, 12), r.randint(3, 6), r.randint(2, 4), RUST[1], 120)
            if r.random() < .2:
                for _ in range(6):
                    s.px(tx + r.randint(0, 15), ty + 15, IVY[r.randint(0, 2)])
    for _ in range(6):  # oil puddles with sheen
        x, y = r.randint(20, W - 20), r.randint(70, H - 10)
        s.ellipse(x, y, r.randint(6, 12), r.randint(3, 5), (36, 40, 50))
        for k in range(4):
            s.px(x - 3 + k * 2, y - 1, [(160, 90, 200), (90, 200, 200), (230, 210, 90), (220, 120, 120)][k])
    # back wall
    s.rect(0, 0, W, 54, (96, 70, 58))
    for x in range(0, W, 4):
        s.rect(x, 0, x, 54, (78, 56, 48))
    for _ in range(30):
        x = r.randint(0, W); s.rect(x, r.randint(0, 20), x + 1, r.randint(30, 54), RUST[1])
    for wx in (40, 170, 300):  # broken windows, light pouring in
        s.rect(wx, 8, wx + 50, 34, (180, 210, 220))
        for k in range(0, 50, 10):
            s.rect(wx + k, 8, wx + k + 1, 34, (60, 50, 44))
        s.rect(wx, 20, wx + 50, 21, (60, 50, 44))
        for k in range(6):
            s.px(wx + r.randint(0, 50), r.randint(8, 34), (60, 50, 44))
        for xx in range(wx, wx + 50, 3):  # ivy coming in through the window
            for k in range(r.randint(2, 14)):
                s.px(xx + (k // 3) % 2, 8 + k, IVY[r.randint(0, 2)])
    s.rect(0, 54, W, 56, METAL[3])

    def pipe_h(y, x0, x1, col=RUST):
        def f():
            s.rect(x0, y - 4, x1, y + 4, col[1]); s.rect(x0, y - 4, x1, y - 2, col[0])
            s.rect(x0, y + 3, x1, y + 4, col[2])
            for x in range(x0, x1, 28):
                s.rect(x, y - 6, x + 3, y + 6, METAL[0]); s.rect(x, y - 6, x + 3, y - 5, METAL[2])
            rr = random.Random(y)
            for _ in range((x1 - x0) // 8):
                xx = rr.randint(x0, x1)
                for k in range(rr.randint(2, 7)):
                    s.px(xx + (k % 2), y + 5 + k, IVY[1])
        s.add(y + 6, f)

    def tank(x, y, rad=18, hgt=40):
        def f():
            s.ellipse(x, y, rad + 2, 5, (40, 44, 50), 120)
            s.rect(x - rad, y - hgt, x + rad, y, METAL[1])
            s.rect(x - rad, y - hgt, x - rad + 5, y, METAL[2])
            s.rect(x + rad - 4, y - hgt, x + rad, y, METAL[3])
            s.ellipse(x, y - hgt, rad, 5, METAL[2]); s.ellipse(x, y - hgt, rad - 3, 3, METAL[0])
            for yy in range(y - hgt + 6, y, 10):
                s.rect(x - rad, yy, x + rad, yy, METAL[3])
            for k in range(0, hgt, 3):
                s.px(x + rad - 8, y - k, METAL[3]); s.px(x + rad - 6, y - k, METAL[3])
            rr = random.Random(x)
            for _ in range(40):
                s.px(x + rr.randint(-rad, rad), y - rr.randint(0, hgt), rr.choice(RUST[:2]))
            for k in range(12):  # creeper
                s.px(x - rad + 3 + int(3 * math.sin(k)), y - k * 3, IVY[1])
        s.add(y, f)

    def conveyor(x0, x1, y):
        def f():
            s.rect(x0, y - 6, x1, y, (52, 54, 60)); s.rect(x0, y - 6, x1, y - 5, (80, 82, 88))
            for x in range(x0, x1, 8):
                s.ellipse(x + 3, y + 1, 2, 2, METAL[0])
            for x in range(x0 + 10, x1 - 10, 34):  # crates left on the belt
                s.rect(x, y - 16, x + 12, y - 6, WOOD[0]); s.rect(x, y - 16, x + 12, y - 15, WOOD[2])
                s.rect(x, y - 11, x + 12, y - 11, WOOD[1]); s.rect(x + 6, y - 16, x + 6, y - 6, WOOD[1])
            for x in range(x0, x1, 6):
                if (x // 6) % 3 == 0:
                    s.px(x, y - 7, IVY[2])
        s.add(y, f)

    pipe_h(64, 0, W)
    tank(40, 140); tank(340, 128, 20, 46)
    conveyor(110, 280, 110)
    s.bush(90, 200, 7, True); s.bush(300, 205, 6); s.tree(372, 220, 12)
    s.rubble(200, 210, 7); s.rubble(260, 150, 5)
    s.spawn(190, 172, 14, beam=True)
    for x, y in ((60, 74), (230, 74), (330, 74)):   # warm work lights still burning
        def f(x=x, y=y):
            for k in range(y - 56):
                s.px(x, 56 + k, (60, 60, 64))
            s.rect(x - 3, y, x + 3, y + 2, (70, 72, 78)); s.rect(x - 2, y + 3, x + 2, y + 3, (255, 220, 140))
        s.add(y, f)
        s.light(x, y + 60, 34, 16, col=(255, 210, 140), strength=60)
    s.sprite(SPI("miner"), "idle_down", 0, 190, 172, shadow=False)
    s.sprite(SPI("scientist"), "walk_right", 2, 236, 196, shadow=False)
    s.sprite(SPI("wolf"), "idle_left", 0, 300, 160, shadow=False)
    s.sprite(HERO, "idle_right", 0, 130, 172)
    s.sprite(NPC("merchant"), "idle_down", 0, 90, 140)
    s.sprite(NPC("scrap"), "idle_left", 1, 252, 128)
    img = s.render(tint=(40, 30, 24, 70), rays=True)
    return caption(img, "THE RUST WORKS - afternoon",
                   "A dead factory floor: light through broken windows, creepers on the tanks. The merchant set up shop where the conveyor stopped.")


# ================================================================ AMUSEMENT PARK
def park():
    s = Scene(404)
    r = s.rnd
    s.grass(flowers=0.004)
    for ty in range(60, 200, 8):  # paver plaza with grass in the gaps
        off = 4 if (ty // 8) % 2 else 0
        for tx in range(30 + off, 350, 8):
            if r.random() < .1:
                continue
            c = r.choice(PAVE)
            s.rect(tx, ty, tx + 6, ty + 6, c); s.rect(tx, ty, tx + 6, ty, sh(c, 14))

    def wheel(cx, base, rad=48):
        def f():
            cy = base - rad - 12
            for lx in (-26, 26):  # A-frame legs
                for t in range(rad + 12):
                    x = cx + lx * t / (rad + 12)
                    s.px(x, base - (rad + 12) + t, METAL[3]); s.px(x + 1, base - (rad + 12) + t, METAL[1])
            for a in range(0, 360, 30):  # spokes
                for t in range(rad):
                    s.px(cx + math.cos(math.radians(a)) * t, cy + math.sin(math.radians(a)) * t, METAL[0])
            for a10 in range(0, 3600, 4):  # rim (with a broken gap)
                a = a10 / 10
                if 300 < a < 330:
                    continue
                s.px(cx + math.cos(math.radians(a)) * rad, cy + math.sin(math.radians(a)) * rad, METAL[2])
                s.px(cx + math.cos(math.radians(a)) * (rad - 1), cy + math.sin(math.radians(a)) * (rad - 1), METAL[1])
            for i, a in enumerate(range(0, 360, 30)):
                if a in (300, 330):
                    continue
                gx, gy = cx + math.cos(math.radians(a)) * rad, cy + math.sin(math.radians(a)) * rad
                col = STRIPE[i % 4] if i % 3 else (150, 150, 150)
                s.rect(gx - 3, gy + 1, gx + 3, gy + 6, col); s.rect(gx - 3, gy + 1, gx + 3, gy + 1, sh(col, 30))
                s.rect(gx - 2, gy + 3, gx + 2, gy + 4, (60, 60, 70))
            s.ellipse(cx, cy, 4, 4, METAL[2])
            for k in range(0, rad + 10, 2):  # vines up the legs
                s.px(cx - 26 + k * 26 / (rad + 12) - 1, base - k, IVY[1])
                s.px(cx + 26 - k * 26 / (rad + 12) + 2, base - k, IVY[2] if k % 4 else IVY[0])
            # fallen gondola
            s.rect(cx + 34, base - 6, cx + 41, base, STRIPE[2]); s.rect(cx + 34, base - 6, cx + 41, base - 6, sh(STRIPE[2], 30))
        s.add(base, f)

    def carousel(cx, base):
        def f():
            s.ellipse(cx, base, 34, 10, (60, 50, 50), 120)
            s.ellipse(cx, base - 2, 32, 9, WOOD[1]); s.ellipse(cx, base - 3, 30, 8, WOOD[2])
            for i, a in enumerate(range(0, 360, 45)):
                px_, py_ = cx + math.cos(math.radians(a)) * 24, base - 3 + math.sin(math.radians(a)) * 6
                for k in range(26):
                    s.px(px_, py_ - k, STRIPE[3])
                if a % 90 == 0:  # horses
                    hy = py_ - 10 - (i % 2) * 2
                    s.rect(px_ - 4, hy, px_ + 4, hy + 3, STRIPE[1]); s.rect(px_ + 3, hy - 3, px_ + 5, hy, STRIPE[1])
                    s.px(px_ - 3, hy + 4, STRIPE[1]); s.px(px_ + 3, hy + 4, STRIPE[1])
            apex = (cx, base - 50)
            for i in range(16):
                a0, a1 = math.radians(i * 22.5), math.radians((i + 1) * 22.5)
                p0 = (cx + math.cos(a0) * 34, base - 30 + math.sin(a0) * 9)
                p1 = (cx + math.cos(a1) * 34, base - 30 + math.sin(a1) * 9)
                col = STRIPE[0] if i % 2 else STRIPE[1]
                for t in range(20):
                    tt = t / 19
                    for u in range(12):
                        uu = u / 11
                        x = apex[0] + (p0[0] + (p1[0] - p0[0]) * uu - apex[0]) * tt
                        y = apex[1] + (p0[1] + (p1[1] - p0[1]) * uu - apex[1]) * tt
                        if math.sin(a0) > -0.2 or tt < .99:
                            s.px(x, y, col)
            s.rect(cx - 1, base - 54, cx, base - 50, STRIPE[3])
            for k in range(14):  # moss on the roof
                s.px(cx - 28 + k * 4, base - 30 + (k % 3), IVY[1])
        s.add(base, f)

    def stall(x, y):
        def f():
            s.rect(x, y - 16, x + 26, y, WOOD[0]); s.rect(x + 2, y - 12, x + 24, y - 6, (60, 44, 34))
            for i in range(0, 28, 4):
                s.rect(x - 1 + i, y - 24, x + 2 + i, y - 17, STRIPE[0] if (i // 4) % 2 else STRIPE[1])
            for i in range(0, 28, 4):
                s.ellipse(x + 1 + i, y - 17, 2, 1, STRIPE[0] if (i // 4) % 2 else STRIPE[1])
            for k in range(8):
                s.px(x + 3 + k * 3, y - 6, FLOW[k % 4])
        s.add(y, f)

    def bunting(x0, y0, x1, y1):
        n_ = x1 - x0
        for t in range(n_):
            if t % 21 == 10:
                yy_ = y0 - 30 + (y1 - y0) * t / n_ + 6 * math.sin(math.pi * t / n_)
                s.light(x0 + t + 1, yy_ + 28, 20, 9, col=(255, 200, 120), strength=90)
        def f():
            for k in (0, 1):
                xx = (x0, x1)[k]; yy = (y0, y1)[k]
                for t in range(30):
                    s.px(xx, yy - t, WOOD[1])
            n = x1 - x0
            for t in range(n):
                x = x0 + t
                y = y0 - 30 + (y1 - y0) * t / n + 6 * math.sin(math.pi * t / n)
                s.px(x, y, (80, 70, 60))
                if t % 7 == 0:
                    c = STRIPE[(t // 7) % 4]
                    s.rect(x, y + 1, x + 3, y + 2, c); s.rect(x + 1, y + 3, x + 2, y + 3, c)
                if t % 21 == 10:   # lanterns
                    s.rect(x, y + 1, x + 2, y + 4, (255, 210, 120))
        s.add(max(y0, y1), f)

    wheel(90, 112)
    carousel(270, 120)
    stall(170, 92); stall(330, 196)
    bunting(130, 150, 230, 140)
    bunting(20, 205, 150, 210)
    s.tree(20, 130, 14); s.tree(370, 92, 15); s.tree(210, 222, 12)
    s.bush(160, 200, 6, True); s.bush(360, 150, 7)
    s.spawn(196, 176, 14, beam=True)
    s.sprite(SPI("child"), "walk_right", 1, 196, 176, shadow=False)
    s.sprite(SPI("farmer"), "idle_left", 1, 230, 186, shadow=False)
    s.sprite(SPI("squirrel"), "hop_right", 2, 300, 150, shadow=False)
    s.sprite(SPI("hare"), "idle_right", 0, 140, 186, shadow=False)
    s.sprite(HERO, "idle_up", 0, 120, 150)
    s.sprite(NPC("radio"), "idle_down", 1, 316, 172)
    return caption(s.render(tint=(90, 50, 130, 85), rays=False),
                   "THE LAST FAIR - dusk",
                   "A half-broken Ferris wheel and a mossy carousel. Lanterns still come on at dusk; visitors' kids log in here most of all.")


from replicant_biomes import cliff_village, seafront  # noqa: E402

if __name__ == "__main__":
  for name, fn in (("biome_cliff_village", cliff_village), ("biome_seafront", seafront),
                 ("biome_dune_city", desert), ("biome_drowned_district", flooded),
                 ("biome_rust_works", factory), ("biome_last_fair", park)):
    fn().save(f"{OUT}/{name}.png")
    print(name)
