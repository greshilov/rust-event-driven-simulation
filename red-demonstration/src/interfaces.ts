export interface GameResult {
  player_name: string;
  score: number;
  ticks_per_sec: number;
}

export interface SignedGameResult {
  game_result: GameResult;
  // Int8Array
  hex_digest: Array<number>;
}

export interface IResult {
  id: number;
  player_uuid: string;
  player_name: string;
  score: number;
  created_at: string;
}

export interface IPlayer {
  uuid: string;
  name: string;
}

export interface IParticleGenerateOptions {
  r?: number;
  density?: number; // Lies between 0..1
  color?: Array<number>; // [255, 0, 0] == red color
  startPoint?: {
    x: number;
    y: number;
  };
  speedLimit?: number;
}
