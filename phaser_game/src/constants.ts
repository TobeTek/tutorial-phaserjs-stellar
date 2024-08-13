const CANVAS_WIDTH = 1024;
const CANVAS_HEIGHT = 768;

enum Colors {
  YELLOW = '0xffff00',
  RICH_GOLD = '0xd4af37',
  DARK_GOLD = '0xbd9258',
  WARM_GOLD = '0xc29b57',
  DEEP_GOLD = '0xb58863',
  LIGHT_GOLD = '0xe8c899',
  METALLIC_GOLD = '0xb8860b',
  CHAMPAGNE_GOLD = '0xd8b07d',
}

export function colorWithHash(color: Colors){
    return color.replace('0x', '#');
}

export { CANVAS_WIDTH, CANVAS_HEIGHT, Colors };
