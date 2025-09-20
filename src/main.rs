use eframe::egui;
use rand::Rng;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Nonograms",
        options,
        Box::new(|_cc| Ok(Box::new(NonogramsApp::new()))),
    )
}

#[derive(PartialEq, Clone, Copy)]
enum CellState {
    Empty,
    Filled,
    Crossed,
}

struct NonogramsApp {
    player_grid: Vec<Vec<CellState>>,   // 玩家的当前状态
    solution_grid: Vec<Vec<CellState>>, // 谜题的正确答案
    row_clues: Vec<Vec<u8>>,
    col_clues: Vec<Vec<u8>>,
    grid_size: usize,
    game_won: bool,
}

impl NonogramsApp {
    /// 构建函数
    fn new() -> Self {
        let grid_size = 5;
        let mut app = Self {
            player_grid: vec![vec![CellState::Empty; grid_size]; grid_size],
            solution_grid: vec![vec![CellState::Empty; grid_size]; grid_size],
            row_clues: Vec::new(),
            col_clues: Vec::new(),
            grid_size,
            game_won: false,
        };

        app.generate_puzzle();
        app
    }

    /// 生成谜底
    fn generate_puzzle(&mut self) {
        // 清空玩家网格
        self.player_grid = vec![vec![CellState::Empty; self.grid_size]; self.grid_size];
        self.game_won = false;

        // 生成随机答案
        let mut rng = rand::rng();
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                self.solution_grid[i][j] = if rng.random_bool(0.5) {
                    CellState::Filled
                } else {
                    CellState::Empty
                };
            }
        }

        // 基于答案计算行和列的提示数字
        self.calculate_clues();
    }

    /// 计算提示数字
    fn calculate_clues(&mut self) {
        self.row_clues.clear();
        self.col_clues.clear();

        // 计算行提示
        for i in 0..self.grid_size {
            let mut clues = Vec::new();
            let mut count = 0;

            for j in 0..self.grid_size {
                if self.solution_grid[i][j] == CellState::Filled {
                    count += 1;
                } else if count > 0 {
                    clues.push(count);
                    count = 0;
                }
            }

            if count > 0 {
                clues.push(count);
            }

            if clues.is_empty() {
                clues.push(0);
            }

            self.row_clues.push(clues);
        }

        // 计算列提示
        for j in 0..self.grid_size {
            let mut clues = Vec::new();
            let mut count = 0;

            for i in 0..self.grid_size {
                if self.solution_grid[i][j] == CellState::Filled {
                    count += 1;
                } else if count > 0 {
                    clues.push(count);
                    count = 0;
                }
            }

            if count > 0 {
                clues.push(count);
            }

            if clues.is_empty() {
                clues.push(0);
            }

            self.col_clues.push(clues);
        }
    }

    /// 检查玩家填充的单元格是否与答案一致
    fn check_win(&mut self) {
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                if (self.player_grid[i][j] == CellState::Filled)
                    != (self.solution_grid[i][j] == CellState::Filled)
                {
                    self.game_won = false;
                    return;
                }
            }
        }
        self.game_won = true;
    }

    /// 点击控制 
    fn handle_cell_click(&mut self, row: usize, col: usize, button: egui::PointerButton) {
        if self.game_won {
            return;
        }

        match button {
            egui::PointerButton::Primary => {
                // 左键点击：切换填充/空白
                self.player_grid[row][col] = match self.player_grid[row][col] {
                    CellState::Empty => CellState::Filled,
                    CellState::Filled => CellState::Empty,
                    CellState::Crossed => CellState::Filled,
                };
            }
            egui::PointerButton::Secondary => {
                // 右键点击：切换交叉/空白
                self.player_grid[row][col] = match self.player_grid[row][col] {
                    CellState::Empty => CellState::Crossed,
                    CellState::Filled => CellState::Crossed,
                    CellState::Crossed => CellState::Empty,
                };
            }
            _ => {}
        }

        self.check_win();
    }
}

impl eframe::App for NonogramsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nonograms");

            if self.game_won {
                ui.label(egui::RichText::new("You WIN!").color(egui::Color32::GREEN));
            }

            if ui.button("New Game").clicked() {
                self.generate_puzzle();
            }

            ui.separator();

            // 绘制列提示
            egui::Grid::new("col_clues_grid")
                .spacing([2.0, 2.0])
                .show(ui, |ui| {
                    // 空单元格用于对齐
                    ui.add_sized([30.0, 30.0], egui::Label::new(""));

                    // 列提示
                    for col in 0..self.grid_size {
                        let clues_text = self.col_clues[col]
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<String>>()
                            .join("\n");

                        ui.vertical(|ui| {
                            ui.add_sized([15.0, 15.0], egui::Label::new(clues_text));
                        });
                    }
                    ui.end_row();

                    // 行提示和游戏网格
                    for row in 0..self.grid_size {
                        // 行提示
                        let clues_text = self.row_clues[row]
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<String>>()
                            .join(" ");

                        ui.add_sized([30.0, 30.0], egui::Label::new(clues_text));

                        // 游戏网格（显示玩家网格）
                        for col in 0..self.grid_size {
                            let cell_response = ui.add_sized(
                                [30.0, 30.0],
                                egui::Button::new("")
                                    .fill(match self.player_grid[row][col] {
                                        CellState::Empty => ui.visuals().extreme_bg_color,
                                        CellState::Filled => egui::Color32::BLACK,
                                        CellState::Crossed => ui.visuals().extreme_bg_color,
                                    })
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY)),
                            );

                            if self.player_grid[row][col] == CellState::Crossed {
                                let rect = cell_response.rect;
                                let painter = ui.painter();
                                painter.line_segment(
                                    [rect.left_top(), rect.right_bottom()],
                                    egui::Stroke::new(2.0, egui::Color32::RED),
                                );
                                painter.line_segment(
                                    [rect.right_top(), rect.left_bottom()],
                                    egui::Stroke::new(2.0, egui::Color32::RED),
                                );
                            }

                            if cell_response.clicked() {
                                self.handle_cell_click(row, col, egui::PointerButton::Primary);
                            } else if cell_response.secondary_clicked() {
                                self.handle_cell_click(row, col, egui::PointerButton::Secondary);
                            }
                        }
                        ui.end_row();
                    }
                });
        });

        ctx.request_repaint();
    }
}
