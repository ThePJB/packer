use minvect::*;
pub const TILE_GRASS: usize = 94;
pub const TILE_MUD: usize = 95;
pub const TILE_FLAGSTONE: usize = 96;
pub const TILE_SAND: usize = 97;
pub const TILE_DIRT: usize = 98;
pub const TILE_LEAFY_GRASS: usize = 99;
pub const TILE_MOSSY_FLAGSTONE: usize = 100;
pub const TREE_MONKEY_PUZZLE: usize = 101;
pub const TREE_WEEPING_WILLOW: usize = 102;
pub const TREE_HEMLOCK_PINE: usize = 103;
pub const TREE_ABIES_ALBA: usize = 104;
pub const TREE_SILVER_FIR: usize = 105;
pub const TREE_STONE_PINE: usize = 106;
pub const TREE_LEBANON_CEDAR: usize = 107;
pub const TREE_SWISS_STONE_PINE: usize = 108;
pub const TREE_RED_CEDAR: usize = 109;

pub const HACK_REGULAR_START: usize = 0;
pub const HACK_REGULAR_END: usize = 93;
pub const TILE_START: usize = 94;
pub const TILE_END: usize = 100;
pub const TREE_START: usize = 101;
pub const TREE_END: usize = 109;

pub const CLIPS: &[Rect] = &[
	rect(0.875,0.072265625,0.020507813,0.024414063,),
	rect(0.9160156,0.18359375,0.01953125,0.032226563,),
	rect(0.9160156,0.15429688,0.017578125,0.024414063,),
	rect(0.98046875,0.119140625,0.018554688,0.032226563,),
	rect(0.9736328,0.25,0.020507813,0.024414063,),
	rect(0.6894531,0.25,0.017578125,0.03125,),
	rect(0.89453125,0.24902344,0.018554688,0.032226563,),
	rect(0.578125,0.25,0.018554688,0.03125,),
	rect(0.74902344,0.25,0.016601563,0.032226563,),
	rect(0.8095703,0.25,0.0126953125,0.040039063,),
	rect(0.8955078,0.21777344,0.01953125,0.03125,),
	rect(0.9550781,0.24902344,0.018554688,0.032226563,),
	rect(0.9345703,0.25195313,0.020507813,0.0234375,),
	rect(0.8613281,0.2763672,0.018554688,0.0234375,),
	rect(0.8613281,0.25195313,0.020507813,0.024414063,),
	rect(0.8955078,0.18554688,0.01953125,0.032226563,),
	rect(0.9355469,0.18847656,0.01953125,0.032226563,),
	rect(0.5371094,0.28027344,0.016601563,0.0234375,),
	rect(0.5205078,0.28027344,0.016601563,0.024414063,),
	rect(0.5966797,0.25,0.018554688,0.03125,),
	rect(0.91308594,0.27734375,0.018554688,0.0234375,),
	rect(0.9345703,0.27539063,0.020507813,0.022460938,),
	rect(0.7246094,0.25,0.024414063,0.022460938,),
	rect(0.8222656,0.25,0.022460938,0.022460938,),
	rect(0.875,0.09667969,0.021484375,0.03125,),
	rect(0.92089844,0.038085938,0.016601563,0.022460938,),
	rect(0.9404297,0.,0.024414063,0.030273438,),
	rect(0.5,0.25,0.01953125,0.030273438,),
	rect(0.97558594,0.21875,0.01953125,0.03125,),
	rect(0.9550781,0.18847656,0.020507813,0.030273438,),
	rect(0.51953125,0.25,0.01953125,0.030273438,),
	rect(0.6152344,0.25,0.018554688,0.030273438,),
	rect(0.93359375,0.12109375,0.020507813,0.03125,),
	rect(0.97558594,0.18847656,0.020507813,0.030273438,),
	rect(0.84472656,0.25,0.016601563,0.030273438,),
	rect(0.70703125,0.25,0.017578125,0.03125,),
	rect(0.8955078,0.06640625,0.022460938,0.030273438,),
	rect(0.63378906,0.25,0.018554688,0.030273438,),
	rect(0.96875,0.08886719,0.022460938,0.030273438,),
	rect(0.875,0.19042969,0.020507813,0.030273438,),
	rect(0.875,0.12792969,0.020507813,0.03125,),
	rect(0.5390625,0.25,0.01953125,0.030273438,),
	rect(0.875,0.,0.022460938,0.038085938,),
	rect(0.91796875,0.08984375,0.022460938,0.030273438,),
	rect(0.93359375,0.15234375,0.020507813,0.03125,),
	rect(0.9453125,0.060546875,0.0234375,0.030273438,),
	rect(0.875,0.22070313,0.01953125,0.03125,),
	rect(0.9404297,0.09082031,0.022460938,0.030273438,),
	rect(0.96484375,0.,0.024414063,0.030273438,),
	rect(0.9404297,0.030273438,0.024414063,0.030273438,),
	rect(0.96484375,0.030273438,0.024414063,0.030273438,),
	rect(0.91503906,0.21582031,0.020507813,0.030273438,),
	rect(0.84472656,0.28027344,0.0107421875,0.0078125,),
	rect(0.65234375,0.25,0.018554688,0.030273438,),
	rect(0.67089844,0.25,0.018554688,0.030273438,),
	rect(0.9355469,0.22070313,0.01953125,0.03125,),
	rect(0.9550781,0.21875,0.020507813,0.030273438,),
	rect(0.91503906,0.24609375,0.01953125,0.03125,),
	rect(0.8955078,0.15429688,0.020507813,0.03125,),
	rect(0.55859375,0.25,0.01953125,0.030273438,),
	rect(0.95410156,0.15722656,0.020507813,0.03125,),
	rect(0.9746094,0.15722656,0.020507813,0.03125,),
	rect(0.875,0.15917969,0.020507813,0.03125,),
	rect(0.8955078,0.14746094,0.016601563,0.00390625,),
	rect(0.6376953,0.28027344,0.022460938,0.01171875,),
	rect(0.66015625,0.28027344,0.022460938,0.009765625,),
	rect(0.95410156,0.12109375,0.0068359375,0.032226563,),
	rect(0.89746094,0.,0.0234375,0.03515625,),
	rect(0.92089844,0.060546875,0.024414063,0.029296875,),
	rect(0.92089844,0.,0.01953125,0.038085938,),
	rect(0.96875,0.060546875,0.024414063,0.028320313,),
	rect(0.6152344,0.28027344,0.022460938,0.0126953125,),
	rect(0.89746094,0.03515625,0.0234375,0.03125,),
	rect(0.5,0.28027344,0.020507813,0.020507813,),
	rect(0.88183594,0.25195313,0.0107421875,0.037109375,),
	rect(0.9892578,0.,0.009765625,0.037109375,),
	rect(0.5722656,0.28125,0.021484375,0.00390625,),
	rect(0.7246094,0.27246094,0.022460938,0.021484375,),
	rect(0.765625,0.25,0.013671875,0.038085938,),
	rect(0.7792969,0.25,0.013671875,0.038085938,),
	rect(0.8964844,0.09667969,0.017578125,0.038085938,),
	rect(0.9628906,0.119140625,0.017578125,0.038085938,),
	rect(0.875,0.038085938,0.020507813,0.034179688,),
	rect(0.99316406,0.037109375,0.0048828125,0.041015625,),
	rect(0.55371094,0.28027344,0.009765625,0.03125,),
	rect(0.99121094,0.08886719,0.0068359375,0.021484375,),
	rect(0.9628906,0.09082031,0.0048828125,0.0126953125,),
	rect(0.8955078,0.13476563,0.0126953125,0.0126953125,),
	rect(0.56347656,0.28027344,0.0087890625,0.015625,),
	rect(0.8222656,0.27246094,0.022460938,0.021484375,),
	rect(0.99121094,0.11035156,0.0068359375,0.0078125,),
	rect(0.9736328,0.27441406,0.022460938,0.021484375,),
	rect(0.9140625,0.12011719,0.01953125,0.034179688,),
	rect(0.79296875,0.25,0.016601563,0.03125,),
	rect(0.625,0.,0.125,0.125,),
	rect(0.75,0.,0.125,0.125,),
	rect(0.,0.125,0.125,0.125,),
	rect(0.125,0.125,0.125,0.125,),
	rect(0.25,0.125,0.125,0.125,),
	rect(0.375,0.125,0.125,0.125,),
	rect(0.5,0.125,0.125,0.125,),
	rect(0.,0.25,0.125,0.09375,),
	rect(0.,0.,0.25,0.125,),
	rect(0.125,0.25,0.125,0.09375,),
	rect(0.625,0.125,0.125,0.125,),
	rect(0.75,0.125,0.125,0.125,),
	rect(0.25,0.,0.1875,0.125,),
	rect(0.4375,0.,0.1875,0.125,),
	rect(0.25,0.25,0.125,0.09375,),
	rect(0.375,0.25,0.125,0.09375,),
];

pub const RECTS: &[PixelRect] = &[
	PixelRect{x:896, y:74, w:21, h:25},
	PixelRect{x:938, y:188, w:20, h:33},
	PixelRect{x:938, y:158, w:18, h:25},
	PixelRect{x:1004, y:122, w:19, h:33},
	PixelRect{x:997, y:256, w:21, h:25},
	PixelRect{x:706, y:256, w:18, h:32},
	PixelRect{x:916, y:255, w:19, h:33},
	PixelRect{x:592, y:256, w:19, h:32},
	PixelRect{x:767, y:256, w:17, h:33},
	PixelRect{x:829, y:256, w:13, h:41},
	PixelRect{x:917, y:223, w:20, h:32},
	PixelRect{x:978, y:255, w:19, h:33},
	PixelRect{x:957, y:258, w:21, h:24},
	PixelRect{x:882, y:283, w:19, h:24},
	PixelRect{x:882, y:258, w:21, h:25},
	PixelRect{x:917, y:190, w:20, h:33},
	PixelRect{x:958, y:193, w:20, h:33},
	PixelRect{x:550, y:287, w:17, h:24},
	PixelRect{x:533, y:287, w:17, h:25},
	PixelRect{x:611, y:256, w:19, h:32},
	PixelRect{x:935, y:284, w:19, h:24},
	PixelRect{x:957, y:282, w:21, h:23},
	PixelRect{x:742, y:256, w:25, h:23},
	PixelRect{x:842, y:256, w:23, h:23},
	PixelRect{x:896, y:99, w:22, h:32},
	PixelRect{x:943, y:39, w:17, h:23},
	PixelRect{x:963, y:0, w:25, h:31},
	PixelRect{x:512, y:256, w:20, h:31},
	PixelRect{x:999, y:224, w:20, h:32},
	PixelRect{x:978, y:193, w:21, h:31},
	PixelRect{x:532, y:256, w:20, h:31},
	PixelRect{x:630, y:256, w:19, h:31},
	PixelRect{x:956, y:124, w:21, h:32},
	PixelRect{x:999, y:193, w:21, h:31},
	PixelRect{x:865, y:256, w:17, h:31},
	PixelRect{x:724, y:256, w:18, h:32},
	PixelRect{x:917, y:68, w:23, h:31},
	PixelRect{x:649, y:256, w:19, h:31},
	PixelRect{x:992, y:91, w:23, h:31},
	PixelRect{x:896, y:195, w:21, h:31},
	PixelRect{x:896, y:131, w:21, h:32},
	PixelRect{x:552, y:256, w:20, h:31},
	PixelRect{x:896, y:0, w:23, h:39},
	PixelRect{x:940, y:92, w:23, h:31},
	PixelRect{x:956, y:156, w:21, h:32},
	PixelRect{x:968, y:62, w:24, h:31},
	PixelRect{x:896, y:226, w:20, h:32},
	PixelRect{x:963, y:93, w:23, h:31},
	PixelRect{x:988, y:0, w:25, h:31},
	PixelRect{x:963, y:31, w:25, h:31},
	PixelRect{x:988, y:31, w:25, h:31},
	PixelRect{x:937, y:221, w:21, h:31},
	PixelRect{x:865, y:287, w:11, h:8},
	PixelRect{x:668, y:256, w:19, h:31},
	PixelRect{x:687, y:256, w:19, h:31},
	PixelRect{x:958, y:226, w:20, h:32},
	PixelRect{x:978, y:224, w:21, h:31},
	PixelRect{x:937, y:252, w:20, h:32},
	PixelRect{x:917, y:158, w:21, h:32},
	PixelRect{x:572, y:256, w:20, h:31},
	PixelRect{x:977, y:161, w:21, h:32},
	PixelRect{x:998, y:161, w:21, h:32},
	PixelRect{x:896, y:163, w:21, h:32},
	PixelRect{x:917, y:151, w:17, h:4},
	PixelRect{x:653, y:287, w:23, h:12},
	PixelRect{x:676, y:287, w:23, h:10},
	PixelRect{x:977, y:124, w:7, h:33},
	PixelRect{x:919, y:0, w:24, h:36},
	PixelRect{x:943, y:62, w:25, h:30},
	PixelRect{x:943, y:0, w:20, h:39},
	PixelRect{x:992, y:62, w:25, h:29},
	PixelRect{x:630, y:287, w:23, h:13},
	PixelRect{x:919, y:36, w:24, h:32},
	PixelRect{x:512, y:287, w:21, h:21},
	PixelRect{x:903, y:258, w:11, h:38},
	PixelRect{x:1013, y:0, w:10, h:38},
	PixelRect{x:586, y:288, w:22, h:4},
	PixelRect{x:742, y:279, w:23, h:22},
	PixelRect{x:784, y:256, w:14, h:39},
	PixelRect{x:798, y:256, w:14, h:39},
	PixelRect{x:918, y:99, w:18, h:39},
	PixelRect{x:986, y:122, w:18, h:39},
	PixelRect{x:896, y:39, w:21, h:35},
	PixelRect{x:1017, y:38, w:5, h:42},
	PixelRect{x:567, y:287, w:10, h:32},
	PixelRect{x:1015, y:91, w:7, h:22},
	PixelRect{x:986, y:93, w:5, h:13},
	PixelRect{x:917, y:138, w:13, h:13},
	PixelRect{x:577, y:287, w:9, h:16},
	PixelRect{x:842, y:279, w:23, h:22},
	PixelRect{x:1015, y:113, w:7, h:8},
	PixelRect{x:997, y:281, w:23, h:22},
	PixelRect{x:936, y:123, w:20, h:35},
	PixelRect{x:812, y:256, w:17, h:32},
	PixelRect{x:640, y:0, w:128, h:128},
	PixelRect{x:768, y:0, w:128, h:128},
	PixelRect{x:0, y:128, w:128, h:128},
	PixelRect{x:128, y:128, w:128, h:128},
	PixelRect{x:256, y:128, w:128, h:128},
	PixelRect{x:384, y:128, w:128, h:128},
	PixelRect{x:512, y:128, w:128, h:128},
	PixelRect{x:0, y:256, w:128, h:96},
	PixelRect{x:0, y:0, w:256, h:128},
	PixelRect{x:128, y:256, w:128, h:96},
	PixelRect{x:640, y:128, w:128, h:128},
	PixelRect{x:768, y:128, w:128, h:128},
	PixelRect{x:256, y:0, w:192, h:128},
	PixelRect{x:448, y:0, w:192, h:128},
	PixelRect{x:256, y:256, w:128, h:96},
	PixelRect{x:384, y:256, w:128, h:96},
];