
// simulation of the game, including the game loop and the game state

// Plan:

//
// AI           Game                           Game
// Human =I/O=> Simulation =players=position=> Simulation (that update player ppos in bevy)
//              Client                         Server

// There is 2 modes for network => PERF and SECU
// PERF transit player position + Other player's one, over network (local or internet)
// SECU transit user/AI input output + Screen (or less), over net (DEFAULT)
// So PERF => Game Simulation Client and Game Simulation Server have to run on the network server (minimal bevy all on net cli)
// So SEC => Game Simulation Client have to run on the actual network client, and Game Simulation Server on the net srv 