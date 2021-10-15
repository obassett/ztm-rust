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

fn print_tile(tile: Tile) {
    use Tile::*;
    match tile {
        Brick(brick @ BrickStyle::Gray) | Brick(brick @ BrickStyle::Red) => {
            println!("The brick color is {:?}", brick)
        }
        Brick(brickstyle) => println!("{:?} brick", brickstyle),
        Water(pressure) if pressure.0 >= 10 => println!("High water pressure!"),
        Water(pressure) => println!("Water pressure level: {:?}", pressure),
        Grass | Dirt | Sand => println!("Ground tile"),
        Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: goldsize,
        }) if goldsize >= 100 => println!("Lots of gold!"),
        _ => (),
    }
}

fn main() {
    let tile1 = Tile::Brick(BrickStyle::Dungeon);
    let tile2 = Tile::Brick(BrickStyle::Red);
    let tile3 = Tile::Grass;
    let tile4 = Tile::Water(Pressure(20));
    let tile5 = Tile::Treasure(TreasureChest {
        content: TreasureItem::SuperPower,
        amount: 1,
    });
    let tile6 = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 150,
    });

    let tiles = vec![tile1, tile2, tile3, tile4, tile5, tile6];

    for tile in tiles {
        print_tile(tile)
    }
}
