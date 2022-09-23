// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn match_treasure(tile: Tile) {
    match tile {
        Tile::Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {:?}", brick)
        }
        Tile::Brick(brick) => println!("{:?} brick", brick),

        Tile::Water(Pressure(10..)) => println!("High water pressure!"),
        Tile::Water(Pressure(level @ u16::MIN..=9)) => {
            println!("Water pressure level: {:?}", level)
        }

        Tile::Grass | Tile::Dirt | Tile::Sand => println!("ground tile"),
        Tile::Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: 100..,
        }) => println!("Lots of gold!"),
        _ => println!("anything else"),
    }
}

fn main() {
    let tile_1 = Tile::Brick(BrickStyle::Dungeon);
    let tile_2 = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 22,
    });

    let tile_3 = Tile::Water(Pressure(4));

    let tile_4 = Tile::Wood;

    match_treasure(tile_1);
    match_treasure(tile_2);
    match_treasure(tile_3);
    match_treasure(tile_4);
}
