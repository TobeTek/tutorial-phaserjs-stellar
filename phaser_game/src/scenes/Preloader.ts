import { Scene } from "phaser";

export class Preloader extends Scene {
  constructor() {
    super("Preloader");
  }

  init() {
    //  We loaded this image in our Boot Scene, so we can display it here
    this.add.image(512, 384, "background");

    //  A simple progress bar. This is the outline of the bar.
    this.add.rectangle(512, 384, 468, 32).setStrokeStyle(1, 0xffffff);

    //  This is the progress bar itself. It will increase in size from the left based on the % of progress.
    const bar = this.add.rectangle(512 - 230, 384, 4, 28, 0xffffff);

    //  Use the 'progress' event emitted by the LoaderPlugin to update the loading bar
    this.load.on("progress", (progress: number) => {
      //  Update the progress bar (our bar is 464px wide, so 100% = 464px)
      bar.width = 4 + 460 * progress;
    });
  }

  preload() {
    this.load.setPath("assets");
    this.load.spritesheet("platforms", "images/platforms.png", {
      frameWidth: 64,
      frameHeight: 16,
    });
    this.load.atlas("coin_atlas", "images/coin.png", "images/coin.json");
  }

  create() {
    this.anims.create({
      key: "turning_coin_anim",
      frames: this.anims.generateFrameNames("coin_atlas", {
        prefix: "TurningCoin",
        suffix: ".png",
        zeroPad: 0,
        start: 1,
        end: 4,
      }),
      repeat: -1,
      frameRate: 3,
    });

    //  Move to the MainMenu. You could also swap this for a Scene Transition, such as a camera fade.
    this.scene.start("MainMenu");
  }
}
