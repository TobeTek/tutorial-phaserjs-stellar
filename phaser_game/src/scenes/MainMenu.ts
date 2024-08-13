import { GameObjects, Scene } from "phaser";
import {
  CANVAS_HEIGHT,
  CANVAS_WIDTH,
  Colors,
  colorWithHash,
} from "../constants";
import { CoinButton, TextButton } from "../sprites/ui/Button";

export class MainMenu extends Scene {
  background: GameObjects.Image;
  blackBackground: GameObjects.Rectangle;
  logo: GameObjects.Image;
  title: GameObjects.Text;
  targetLoadingText = "Loading...";
  loadingText: GameObjects.Text;
  platforms: GameObjects.Group;
  connectWalletDialog: GameObjects.Group;
  connectWalletButton: TextButton;
  loadingSpinner: GameObjects.Sprite;

  constructor() {
    super("MainMenu");
  }

  create() {
    this.connectWalletDialog = this.add.group();
    this.createBackground();
    this.createGameUI();
    this.createConnectWalletUI();
  }

  createBackground() {
    // Add background image
    this.background = this.add.image(
      CANVAS_WIDTH / 2,
      CANVAS_HEIGHT / 2,
      "background"
    );

    // Add flying platforms
    const noFlyingPlatforms = 10;
    const maxPlatformSpeed = 15;
    this.platforms = this.add.group();

    for (let index = 0; index < noFlyingPlatforms; index++) {
      const randomPlatformFrame = Math.floor(Math.random() * 4);
      const randomSpeed = Math.floor(Math.random() * maxPlatformSpeed);

      const randY = CANVAS_HEIGHT * Math.random() + 1;
      const frame = this.add.sprite(
        -10,
        randY,
        "platforms",
        randomPlatformFrame
      );
      frame.scale = 5 * Math.random();
      frame.alpha = 0.25;
      frame.update = function (_time: number, _delta: number) {
        this.x += randomSpeed;
        if (this.x >= CANVAS_WIDTH + 50) {
          this.x = -10;
        }
      };
      this.platforms.add(frame);
    }
  }

  createGameUI() {
    this.add
      .text(CANVAS_WIDTH / 2, 100, "Tap to Claim!", {
        fontFamily: "PixelOperator",
        fontSize: 52,
        color: "#ffffff",
        stroke: "#ffffff",
        strokeThickness: 1,
        align: "center",
      })
      .setOrigin(0.5);

    const coin = this.add.sprite(
      CANVAS_WIDTH / 2 - 100,
      200,
      "coin_atlas",
      "TurningCoin1.png"
    );
    coin.scale = 0.2;
    coin.anims.play("turning_coin_anim");

    this.add
      .text(CANVAS_WIDTH / 2 + 50, 200, "001000", {
        fontFamily: "PixelOperator",
        fontSize: 40,
        color: "#ffffff",
        stroke: "#bd9258",
        strokeThickness: 2,
        align: "center",
      })
      .setOrigin(0.5);

    const coinButton = new CoinButton(this, CANVAS_WIDTH / 2, 500, () => {
      console.log("Hello from btn callback");
      this.loadingText.setVisible(true);
    });
    coinButton.setOrigin(0.5);
  }

  createConnectWalletUI() {
    this.blackBackground = this.add
      .rectangle(0, 0, 1e6, 1e6, 0x000000, 0.95)
      .setInteractive();

    const pointerText1 = this.add
      .text(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 - 100, "🔽", {
        fontSize: 50,
      })
      .setOrigin(0.5);
    const pointerText2 = this.add
      .text(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 + 100, "🔼", {
        fontSize: 50,
      })
      .setOrigin(0.5);

    this.connectWalletButton = new TextButton(
      this,
      CANVAS_WIDTH / 2,
      CANVAS_HEIGHT / 2,
      "Connect Wallet",
      {
        fontFamily: "PixelOperator",
        fontSize: 40,
        color: colorWithHash(Colors.DARK_GOLD),
        stroke: colorWithHash(Colors.YELLOW),
        strokeThickness: 3,
        align: "center",
      },
      () => this.clickConnectWallet()
    ).setOrigin(0.5);
    this.connectWalletDialog.add(pointerText1);
    this.connectWalletDialog.add(pointerText2);
    this.connectWalletDialog.add(this.connectWalletButton);

    this.loadingText = this.add
      .text(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2, "Loading...", {
        fontFamily: "PixelOperator",
        fontSize: 30,
      })
      .setOrigin(0.5);
    const fx = this.loadingText.preFX?.addReveal();
    this.tweens.add({
      targets: fx,
      progress: 1,
      hold: 500,
      duration: 2000,
      repeat: -1,
    });
    this.loadingText.setVisible(false);
  }

  update(_time: number, _delta: number) {
    for (const frame of this.platforms.getChildren()) {
      frame.update();
    }
  }

  clickConnectWallet() {
    this.connectWalletDialog.setActive(false);
    this.connectWalletDialog.setVisible(false);
    this.blackBackground.setVisible(false);
    this.loadingText.setVisible(false);
  }
}
