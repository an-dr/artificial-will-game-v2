import json, sys
from PIL import Image, ImageDraw
BG=(58,64,82); S=3; C=96
for name in ("scrap","radio","merchant","gardener"):
    sheet=Image.open(f"npc/npc_{name}.png"); meta=json.load(open(f"npc/npc_{name}.json"))
    anims={a:[sheet.crop((x,y,x+w,y+h)) for x,y,w,h in m["rects"]] for a,m in meta["animations"].items()}
    fps={a:m["fps"] for a,m in meta["animations"].items()}
    def fr(im):
        bg=Image.new("RGBA",im.size,BG+(255,)); bg.alpha_composite(im)
        return bg.resize((C*S,C*S),Image.NEAREST).convert("RGB")
    for a,frames in anims.items():
        imgs=[fr(f) for f in frames]
        imgs[0].save(f"gifs/single/{name}_{a}.gif",save_all=True,append_images=imgs[1:],duration=1000//fps[a],loop=0)
    # overview: 3x3 grid, 24-tick timeline
    cols=3; rows=3; T=24; cs=C*2
    out=[]
    for t in range(T):
        cv=Image.new("RGB",(cols*cs,rows*(cs+14)),BG); d=ImageDraw.Draw(cv)
        for i,(a,frames) in enumerate(anims.items()):
            idx=(t*fps[a]//12)%len(frames)
            bg=Image.new("RGBA",frames[0].size,BG+(255,)); bg.alpha_composite(frames[idx])
            x,y=(i%cols)*cs,(i//cols)*(cs+14)
            cv.paste(bg.resize((cs,cs),Image.NEAREST).convert("RGB"),(x,y))
            d.text((x+6,y+cs),a,fill=(230,230,235))
        out.append(cv)
    out[0].save(f"gifs/{name}_all.gif",save_all=True,append_images=out[1:],duration=1000//12,loop=0)
print("done")
