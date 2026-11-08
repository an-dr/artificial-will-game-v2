"""TV-head robot sprite sheet generator.

Front (down) view is built from the user's 4-frame reference sprite; back, side,
attack, hurt, death and hold poses are derived/drawn in the same palette and
2px-outline style. Usage: python3 tv_robot.py <reference.png> <outdir>
"""
import json
import random
import sys
from PIL import Image, ImageDraw

REF, OUT = sys.argv[1], sys.argv[2]
CELL, O = 96, 16          # 96x96 cells, 64x64 robot box at (16,16)
FEET = (48, 78)

P = {
    "a": (11, 11, 15), "#": (50, 62, 79), "b": (17, 128, 113), "c": (29, 168, 149),
    "d": (207, 255, 112), "e": (39, 39, 54), "f": (15, 95, 115), "g": (150, 66, 83),
    "h": (255, 140, 140), "i": (235, 86, 75), "j": (33, 31, 52), "k": (31, 102, 153),
    "w": (244, 244, 244), "s": (140, 140, 150),
}
RGBA = {k: v + (255,) for k, v in P.items()}

ref = Image.open(REF).convert("RGBA")
SRC = [ref.crop((64 * k, 0, 64 * k + 64, 64)) for k in range(4)]
HD = [0, 1, 0, 1]  # head bob per reference frame


def blank(w=64, h=64):
    return Image.new("RGBA", (w, h), (0, 0, 0, 0))


def rows(img, y0, y1):
    out = blank(img.width, img.height)
    out.paste(img.crop((0, y0, img.width, y1 + 1)), (0, y0))
    return out


def setpx(img, x, y, c):
    if 0 <= x < img.width and 0 <= y < img.height:
        img.putpixel((x, y), RGBA[c] if isinstance(c, str) else c)


def rect(img, x0, y0, x1, y1, c):
    for y in range(y0, y1 + 1):
        for x in range(x0, x1 + 1):
            setpx(img, x, y, c)


def erase(img, x0, y0, x1, y1):
    rect(img, x0, y0, x1, y1, (0, 0, 0, 0))


def mirror(img, shift=0):
    m = img.transpose(Image.FLIP_LEFT_RIGHT)
    out = blank(img.width, img.height)
    out.paste(m, (shift, 0))
    return out


def outline(img, width=2, light_top_left=False):
    """Add a `width`-px outline around opaque pixels (in place, returns img)."""
    src = img.copy()
    W, H = img.size
    op = lambda x, y: 0 <= x < W and 0 <= y < H and src.getpixel((x, y))[3] > 0
    for y in range(H):
        for x in range(W):
            if op(x, y):
                continue
            near = any(op(x + dx, y + dy) for dx in range(-width, width + 1)
                       for dy in range(-width, width + 1) if abs(dx) + abs(dy) <= width + (width > 1))
            if not near:
                continue
            col = "a"
            if light_top_left and any(op(x + k, y) or op(x, y + k) for k in range(1, width + 1)):
                col = "#"
            setpx(img, x, y, col)
    return img


def tint(img, c="w"):
    out = img.copy()
    for y in range(img.height):
        for x in range(img.width):
            p = img.getpixel((x, y))
            if p[3] and p[:3] != P["a"]:
                out.putpixel((x, y), RGBA[c])
    mask = blank(out.width, out.height)
    for y in range(out.height):
        for x in range(out.width):
            if out.getpixel((x, y))[3] and out.getpixel((x, y))[:3] == P[c]:
                mask.putpixel((x, y), RGBA[c])
    outline(mask, 2)
    return mask


# ---------------------------------------------------------------- faces ----
SX, SY, SW, SH = 19, 7, 26, 23  # screen interior


def face(head, kind, seed=0):
    h = head.copy()
    if kind == "normal":
        return h
    s = lambda x, y, c: setpx(h, SX + x, SY + y, c)
    bg = "e" if kind in ("off", "crt") else "d"
    rect(h, SX, SY, SX + SW - 1, SY + SH - 1, bg)
    eyes = (2, 14)
    if kind == "blink":
        for ex in eyes:
            for x in range(ex + 1, ex + 8):
                s(x, 10, "e"); s(x, 11, "e")
        for x in range(10, 16):
            s(x, 20, "e"); s(x, 21, "e")
    elif kind == "angry":
        for x in range(2, 10):           # left eye, inner (right) side lower
            for y in range(5 + (x - 2) // 2, 15):
                s(x, y, "e")
        for x in range(15, 23):          # right eye mirrored
            for y in range(5 + (22 - x) // 2, 15):
                s(x, y, "e")
        for x in range(8, 18):
            s(x, 19, "e"); s(x, 20, "e")
    elif kind == "hurt":
        for ex in eyes:
            for dy in range(8):
                for dx in range(8):
                    if abs(dx - dy) <= 1 or abs(dx + dy - 7) <= 1:
                        s(ex + 1 + dx, 4 + dy, "e")
        rect(h, SX + 10, SY + 17, SX + 15, SY + 21, "e")
        rect(h, SX + 12, SY + 18, SX + 13, SY + 20, "d")
    elif kind == "happy":
        for ex in eyes:
            for dx in range(8):
                py = 5 + int(abs(dx - 3.5))
                s(ex + 1 + dx, py + 3 - 2 * (abs(dx - 3.5) > 2) * 0, "e")
                s(ex + 1 + dx, py + 4, "e")
        for x in range(9, 17):
            s(x, 20, "e")
        s(8, 19, "e"); s(17, 19, "e")
    elif kind == "static":
        rnd = random.Random(seed)
        for y in range(0, SH, 2):
            for x in range(0, SW, 2):
                c = rnd.choice("deedw")
                rect(h, SX + x, SY + y, min(SX + x + 1, SX + SW - 1), min(SY + y + 1, SY + SH - 1), c)
    elif kind == "crt":
        for x in range(3, SW - 3):
            s(x, 11, "d")
        for x in range(9, SW - 9):
            s(x, 11, "w")
    return h


# ---------------------------------------------------------- front parts ----
def head_front(k=0):
    return _shift(rows(SRC[k], 0, 36 + HD[k]), 0, -HD[k])


def _shift(img, dx, dy):
    out = blank(img.width, img.height)
    out.paste(img, (dx, dy))
    return out


def body_front(k=0):
    """Scarf + torso + arm (+ tail) of reference frame k, rows 37..56."""
    b = rows(SRC[0], 37, 37)  # scarf top row (hidden under bobbed head anyway)
    b.alpha_composite(rows(SRC[k], 38, 56))
    return b


def legs(k=0):
    return rows(SRC[k], 57, 63)


def remove_arm(body):
    b = body.copy()
    erase(b, 45, 45, 52, 63)
    return b


def back_head():
    h = head_front(0)
    rect(h, SX - 2, SY - 2, SX + SW + 1, SY + SH + 1, "b")
    for y in range(SY - 2, SY + SH + 2):         # dither like the casing
        for x in range(SX - 2, SX + 6):
            if (x + y) % 2 == 0:
                setpx(h, x, y, "c")
    panel = blank()
    rect(panel, 22, 10, 42, 27, "#")
    for y in (13, 17, 21, 25):
        rect(panel, 25, y, 39, y, "j")
    outline(panel, 1)
    h.alpha_composite(panel)
    rect(h, 30, 30, 34, 33, "a")                  # cable plug
    rect(h, 31, 31, 33, 32, "k")
    return h


def back_body(k=0):
    b = body_front(k)
    rect(b, 21, 45, 42, 53, "#")                  # cover chest panel
    pack = blank()
    rect(pack, 24, 46, 38, 52, "k")
    rect(pack, 24, 46, 38, 46, "c")
    rect(pack, 29, 48, 33, 50, "d")
    outline(pack, 1)
    b.alpha_composite(pack)
    return mirror(b, shift=1)


# ------------------------------------------------------------ side parts ---
def side_head(dy=0, glow="d"):
    L = blank()
    # tube bump (back), CRT box, front bezel
    rect(L, 15, 11, 20, 25, "f"); rect(L, 15, 11, 20, 12, "b")
    rect(L, 21, 4, 39, 32, "b")
    for y in range(4, 7):
        for x in range(21, 40):
            if (x + y) % 2 == 0:
                setpx(L, x, y, "c")
    rect(L, 21, 30, 39, 32, "f")
    for y in (10, 15, 20, 25):
        rect(L, 24, y, 36, y + 1, "f")
    rect(L, 40, 2, 46, 34, "b")
    rect(L, 40, 2, 46, 3, "c")
    rect(L, 40, 2, 40, 34, "f")
    rect(L, 45, 7, 46, 29, glow)                  # screen edge = facing cue
    outline(L, 2, light_top_left=True)
    return _shift(L, 0, dy)


def side_body(tail_sway=0, arm=(0, 9), far_leg=(0, 0), near_leg=(0, 0),
              arm_fn=None):
    """arm=(dx,dy) hand offset from shoulder; legs=(x offset, lift)."""
    out = blank()

    def layer():
        return blank()

    def put(L, ol=2):
        outline(L, ol)
        out.alpha_composite(L)

    def leg(xoff, lift, far):
        L = layer()
        c1, c2, c3 = ("i", "g", "g") if far else ("h", "i", "g")
        x0 = 29 + xoff
        for y in range(57, 61 - lift):
            rect(L, x0, y, x0 + 1, y, c1)
            rect(L, x0 + 2, y, x0 + 3, y, c2 if y < 59 else c3)
            rect(L, x0 + 4, y, x0 + 5, y, c3)
        return L

    # scarf tail (behind)
    L = layer()
    for t in range(7):
        x = int(23 - t * 1.4 - tail_sway * t / 6)
        y = 41 + t * 2
        rect(L, x, y, x + 3, y + 1, "i" if t < 5 else "g")
        setpx(L, x, y, "h")
    put(L, 1)
    put(leg(far_leg[0], far_leg[1], True))
    # torso
    L = layer()
    rect(L, 24, 44, 40, 55, "#")
    rect(L, 24, 44, 25, 55, "j")
    rect(L, 24, 55, 40, 55, "j")
    rect(L, 38, 46, 38, 52, "k"); rect(L, 39, 46, 39, 52, "b"); rect(L, 40, 47, 40, 51, "g")
    put(L)
    # scarf wrap
    L = layer()
    rect(L, 22, 38, 42, 43, "i")
    rect(L, 22, 38, 42, 38, "g")
    rect(L, 22, 39, 28, 39, "h")
    rect(L, 22, 42, 42, 43, "g")
    rect(L, 20, 40, 23, 43, "i"); setpx(L, 20, 40, "h")   # knot at the back
    put(L, 1)
    put(leg(near_leg[0], near_leg[1], False))
    # arm (front)
    L = layer()
    if arm_fn:
        arm_fn(L)
    else:
        sx, sy = 31, 45
        hx, hy = sx + arm[0], sy + arm[1]
        n = max(abs(hx - sx), abs(hy - sy), 1)
        for t in range(n + 1):
            x = round(sx + (hx - sx) * t / n)
            y = round(sy + (hy - sy) * t / n)
            rect(L, x, y, x + 1, y + 1, "#")
        rect(L, hx - 1, hy, hx + 2, hy + 2, "#")
        rect(L, 30, 44, 33, 46, "#")
    put(L, 1)
    return out


# ----------------------------------------------------------- composing -----
def frame():
    return blank(CELL, CELL)


def paste(fr, part, dx=0, dy=0):
    fr.alpha_composite(_shift(part, dx, dy), (O, O))


def fx_layer():
    return blank(CELL, CELL)


def fx_px(L, x, y, c):  # local robot coords
    setpx(L, x + O, y + O, c)


def fx_rect(L, x0, y0, x1, y1, c):
    rect(L, x0 + O, y0 + O, x1 + O, y1 + O, c)


def flash(L, cx, cy, big=True):
    fx_rect(L, cx - 2, cy - 2, cx + 1, cy + 1, "w")
    r = 6 if big else 4
    for d in range(3, r + 1, 1):
        for sx, sy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            fx_px(L, cx + sx * d, cy + sy * d, "d")
            fx_px(L, cx + sx * d - (sy != 0), cy + sy * d - (sx != 0), "d")
    for sx, sy in ((1, 1), (-1, -1), (1, -1), (-1, 1)):
        fx_rect(L, cx + sx * 4 - 1, cy + sy * 4 - 1, cx + sx * 4, cy + sy * 4, "c")


def front_frame(k=0, kind="normal", back=False, arm=None, body=None, head_dy=None,
                head_dx=0, leg_k=None):
    fr = frame()
    hd = HD[k] if head_dy is None else head_dy
    lg = legs(k if leg_k is None else leg_k)
    if back:
        lg = mirror(lg, shift=1)
    paste(fr, lg)
    b = body if body is not None else (back_body(k) if back else body_front(k))
    paste(fr, b)
    h = back_head() if back else face(head_front(0), kind, seed=k)
    paste(fr, h, head_dx, hd)
    if arm is not None:
        fr.alpha_composite(arm)
    return fr


def side_frame(head_dy=0, glow="d", **kw):
    fr = frame()
    body = side_body(**kw)
    fr.alpha_composite(body, (O, O))
    paste(fr, side_head(0, glow), 0, head_dy)
    return fr


def arm_layer(points, hand=True):
    """Thin robot arm (2px core + 2px outline) through local points."""
    L = blank(CELL, CELL)
    for (x0, y0), (x1, y1) in zip(points, points[1:]):
        n = max(abs(x1 - x0), abs(y1 - y0), 1)
        for t in range(n + 1):
            x = round(x0 + (x1 - x0) * t / n) + O
            y = round(y0 + (y1 - y0) * t / n) + O
            rect(L, x, y, x + 1, y + 1, "#")
    if hand:
        x, y = points[-1]
        rect(L, x - 1 + O, y - 1 + O, x + 2 + O, y + 2 + O, "#")
    return outline(L, 2)


# ------------------------------------------------------------ animations ---
def anim_idle(d):
    if d == "down":
        return [front_frame(0), front_frame(0, kind="blink", head_dy=1)]
    if d == "up":
        return [front_frame(0, back=True), front_frame(0, back=True, head_dy=1)]
    return [side_frame(near_leg=(2, 0), far_leg=(-2, 0)),
            side_frame(head_dy=1, near_leg=(2, 0), far_leg=(-2, 0))]


def anim_walk(d):
    if d == "down":
        return [front_frame(k) for k in range(4)]
    if d == "up":
        return [front_frame(k, back=True) for k in range(4)]
    cfg = [dict(near_leg=(5, 0), far_leg=(-5, 0), arm=(-4, 8), tail_sway=0),
           dict(near_leg=(1, 2), far_leg=(-1, 0), arm=(0, 9), tail_sway=2, head_dy=1),
           dict(near_leg=(-5, 0), far_leg=(5, 0), arm=(4, 8), tail_sway=0),
           dict(near_leg=(1, 0), far_leg=(-1, 2), arm=(0, 9), tail_sway=-1, head_dy=1)]
    return [side_frame(**c) for c in cfg]


def anim_attack(d):
    out = []
    if d == "down":
        nb = remove_arm(body_front(0))
        out.append(front_frame(0, "normal", body=nb, arm=arm_layer([(48, 45), (50, 40)])))
        for i in (1, 2):
            fr = front_frame(0, "normal", body=nb, arm=arm_layer([(48, 45), (48, 70)]))
            L = fx_layer()
            if i == 1:
                flash(L, 49, 77)
            else:
                fx_rect(L, 47, 73, 50, 78, "d"); fx_rect(L, 48, 73, 49, 78, "w")
            fr.alpha_composite(L); out.append(fr)
        out.append(front_frame(0, "normal"))
    elif d == "up":
        nb = back_body(0)
        erase(nb, 10, 45, 18, 63)
        out.append(front_frame(0, back=True, body=nb, arm=arm_layer([(15, 45), (12, 50)])))
        for i in (1, 2):
            fr = front_frame(0, back=True, body=nb,
                             arm=arm_layer([(15, 45), (8, 38), (8, -6)]))
            L = fx_layer()
            if i == 1:
                flash(L, 9, -11)
            else:
                fx_rect(L, 7, -14, 10, -9, "d"); fx_rect(L, 8, -14, 9, -9, "w")
            fr.alpha_composite(L); out.append(fr)
        out.append(front_frame(0, back=True))
    else:
        out.append(side_frame(arm=(-6, 4), near_leg=(2, 0), far_leg=(-2, 0)))
        for i in (1, 2):
            fr = side_frame(near_leg=(4, 0), far_leg=(-4, 0),
                            arm_fn=lambda L: None)
            fr.alpha_composite(arm_layer([(31, 45), (60, 45)]))
            L = fx_layer()
            if i == 1:
                flash(L, 68, 46)
            else:
                fx_rect(L, 64, 44, 76, 47, "d"); fx_rect(L, 64, 45, 76, 46, "w")
            fr.alpha_composite(L); out.append(fr)
        out.append(side_frame(near_leg=(2, 0), far_leg=(-2, 0)))
    return out


def anim_hurt(d):
    knock = {"down": (0, -3), "up": (0, 3), "right": (-3, 0)}[d]
    if d == "down":
        base = front_frame(0, "hurt")
    elif d == "up":
        base = front_frame(0, back=True)
    else:
        base = side_frame(near_leg=(2, 0), far_leg=(-2, 0))
    return [_shift(tint(base), *knock), base]


def anim_death():
    out = [tint(front_frame(0, "hurt"))]
    fr = front_frame(0, "static")
    L = fx_layer()
    for x, y in ((6, 10), (58, 4), (54, 50), (8, 52)):
        flash(L, x, y, big=False)
    fr.alpha_composite(L); out.append(fr)
    nb = remove_arm(body_front(0))
    droop = arm_layer([(48, 45), (50, 58)])
    out.append(front_frame(0, "static", body=nb, arm=droop, head_dy=3, head_dx=1))
    out.append(front_frame(0, "crt", body=nb, arm=droop, head_dy=8, head_dx=2))
    fr = front_frame(0, "off", body=nb, arm=droop, head_dy=14, head_dx=2)
    L = fx_layer()
    for x, y in ((14, 30), (50, 34), (32, 20)):
        flash(L, x, y)
    fx_rect(L, 26, 6, 36, 10, "s"); fx_rect(L, 30, 2, 34, 5, "s")
    fr.alpha_composite(L); out.append(fr)
    # final: TV head on the ground on top of the scrap
    fr = frame()
    paste(fr, _shift(body_front(0), 0, 6))
    paste(fr, face(head_front(0), "off"), 2, 24)
    L = fx_layer()
    fx_rect(L, 34, 14, 37, 17, "s"); fx_rect(L, 38, 9, 40, 11, "s"); fx_px(L, 36, 5, "s")
    fr.alpha_composite(L); out.append(fr)
    return out


def anim_hold():
    nb = remove_arm(body_front(0))
    erase(nb, 10, 43, 18, 63)  # hide scarf tail so left arm reads
    arms = arm_layer([(48, 45), (55, 38), (55, 0), (42, -7)])
    arms.alpha_composite(arm_layer([(15, 45), (8, 38), (8, 0), (21, -7)]))
    fr = front_frame(0, "happy", body=nb)
    item = blank(CELL, CELL)  # placeholder item: a glowing chip
    rect(item, 24 + O, -14 + O, 39 + O, -3 + O, "k")
    rect(item, 27 + O, -11 + O, 36 + O, -6 + O, "d")
    for x in range(26, 38, 3):
        rect(item, x + O, -16 + O, x + O + 1, -15 + O, "#")
        rect(item, x + O, -2 + O, x + O + 1, -1 + O, "#")
    outline(item, 2)
    fr.alpha_composite(item)
    fr.alpha_composite(arms)
    return [fr]


# ---------------------------------------------------------------- jump -----
LEG_TOP = O + 57


def crouch(fr, n=3):
    """Squash: everything above the legs drops n px, legs get shorter."""
    out = fr.copy()
    top = fr.crop((0, 0, CELL, LEG_TOP))
    erase(out, 0, 0, CELL - 1, LEG_TOP + n - 1)
    out.alpha_composite(top, (0, n))
    return out


def shadow(width):
    L = blank(CELL, CELL)
    cx, cy = FEET[0], O + 62
    ImageDraw.Draw(L).ellipse((cx - width // 2, cy - 3, cx + width // 2, cy + 3),
                              fill=P["a"] + (90,))
    return L


def airborne(fr, h, tuck=2):
    body = crouch(fr, tuck)               # legs pulled up a bit in the air
    out = shadow(max(14, 34 - h))
    out.alpha_composite(_shift(body, 0, -h))
    return out


def anim_jump(d):
    base = {"down": lambda: front_frame(0),
            "up": lambda: front_frame(0, back=True),
            "right": lambda: side_frame(near_leg=(2, 0), far_leg=(-2, 0))}[d]()
    grounded = lambda f: (lambda s: (s.alpha_composite(f), s)[1])(shadow(34))
    return [grounded(crouch(base, 3)), airborne(base, 10), airborne(base, 16, tuck=3),
            airborne(base, 10), grounded(crouch(base, 4)), grounded(base)]


# ------------------------------------------------------------- assemble ----
rows_ = []
for name, fn in (("idle", anim_idle), ("walk", anim_walk), ("attack", anim_attack),
                 ("hurt", anim_hurt)):
    right = fn("right")
    rows_ += [(f"{name}_down", fn("down")), (f"{name}_up", fn("up")),
              (f"{name}_left", [mirror(f, shift=1) for f in right]),
              (f"{name}_right", right)]
rows_ += [("death", anim_death()), ("hold_item", anim_hold())]
_jr = anim_jump("right")
rows_ += [("jump_down", anim_jump("down")), ("jump_up", anim_jump("up")),
          ("jump_left", [mirror(f, shift=1) for f in _jr]), ("jump_right", _jr)]

FPS = {"idle": 3, "walk": 8, "attack": 12, "hurt": 10, "death": 8, "hold_item": 1, "jump": 12}
LOOP = {"idle": True, "walk": True}
key = lambda n: "hold_item" if n == "hold_item" else n.split("_")[0]

cols = max(len(f) for _, f in rows_)
sheet = blank(cols * CELL, len(rows_) * CELL)
meta = {"image": "tv_robot_sheet.png", "frame_size": [CELL, CELL],
        "origin_feet": list(FEET), "robot_box": [O, O, 64, 64], "animations": {}}
for r, (name, frames) in enumerate(rows_):
    for c, im in enumerate(frames):
        sheet.paste(im, (c * CELL, r * CELL))
    meta["animations"][name] = {
        "row": r, "frames": len(frames), "fps": FPS[key(name)],
        "loop": LOOP.get(key(name), False),
        "rects": [[c * CELL, r * CELL, CELL, CELL] for c in range(len(frames))]}

sheet.save(f"{OUT}/tv_robot_sheet.png")
sheet.resize((sheet.width * 2, sheet.height * 2), Image.NEAREST).save(f"{OUT}/tv_robot_sheet_x2.png")
json.dump(meta, open(f"{OUT}/tv_robot_sheet.json", "w"), indent=1)

# animated preview
S, GC = 2, 6
GR = (len(rows_) + GC - 1) // GC
gif = []
for t in range(48):
    cv = Image.new("RGBA", (GC * CELL * S, GR * (CELL * S + 14)), (58, 64, 82, 255))
    dr = ImageDraw.Draw(cv)
    for i, (name, frames) in enumerate(rows_):
        k = key(name)
        idx = t * FPS[k] // 24
        idx = idx % len(frames) if LOOP.get(k) else min(idx % (len(frames) + 4), len(frames) - 1)
        x, y = (i % GC) * CELL * S, (i // GC) * (CELL * S + 14)
        cv.alpha_composite(frames[idx].resize((CELL * S, CELL * S), Image.NEAREST), (x, y))
        dr.text((x + 4, y + CELL * S), name, fill=(230, 230, 235, 255))
    gif.append(cv.convert("RGB"))
gif[0].save(f"{OUT}/tv_robot_preview.gif", save_all=True, append_images=gif[1:],
            duration=1000 // 24, loop=0)
print(sheet.size)
