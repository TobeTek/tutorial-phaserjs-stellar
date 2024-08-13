import Phaser, { Scene } from "phaser";

export class TextButton extends Phaser.GameObjects.Text {
  constructor(
    scene: Scene,
    x: integer,
    y: integer,
    text: string,
    style: Phaser.Types.GameObjects.Text.TextStyle,
    callback: CallableFunction,
  ) {
    super(scene, x, y, text, style);
    scene.add.existing(this);

    this.setInteractive({ useHandCursor: true })
      .on("pointerover", () => this.enterButtonHoverState())
      .on("pointerout", () => this.enterButtonRestState())
      .on("pointerdown", () => this.enterButtonActiveState())
      .on("pointerup", () => {
        this.enterButtonHoverState();
        callback();
      });
  }

  enterButtonHoverState() {
    this.setStyle({ fill: "#ff0" });
  }

  enterButtonRestState() {
    this.setStyle({ fill: "#0f0" });
  }

  enterButtonActiveState() {
    this.setStyle({ fill: "#0ff" });
  }
}

export class CoinButton extends Phaser.GameObjects.Sprite {
  constructor(
    scene: Scene,
    x: integer,
    y: integer,
    callback: CallableFunction
  ) {
    super(scene, x, y, 'coin_atlas', 'Coin1.png');
    scene.add.existing(this);
    // const coinSprite = scene.add.sprite(x, y, "coin_atlas", "Coin1.png");
    this.setInteractive({ useHandCursor: true })
      .on("pointerover", () => this.enterButtonHoverState())
      .on("pointerout", () => this.enterButtonRestState())
      .on("pointerdown", () => this.enterButtonActiveState())
      .on("pointerup", () => {
        this.enterButtonHoverState();
        callback();
      });
  }

  enterButtonHoverState() {
    console.log('Hover');
  }

  enterButtonRestState() {
    console.log('Rest');
  }

  enterButtonActiveState() {
    console.log("Active!");
    this.scene.tweens.add({
      targets: this,
      rotation: 180,
      scaleY: 1.5,
      scaleX: 1.5,
      ease: "Power1",
      duration: 1000,
      repeat: 0,
      yoyo: true,
      onComplete: () => {
        this.alpha= 0.5;
      },
      callbackScope: this,
    });
  }
}
