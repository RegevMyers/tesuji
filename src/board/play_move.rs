use crate::common::prolog::*;

use crate::board::{Board, Dimensions, Group, Point};

impl Board {
    pub(super) fn play_move(&mut self, color: Color, point: Point) -> Result<(), Error> {
        log::trace(&format!("Move | {color:?} @ [{point:?}]"));

        self.board[point].stone = Some(color);

        let adjacent_points = self.get_adjacent_points(point);
        let adjacent_groups = adjacent_points.into_iter().map(|point| self.get_containing_group(point)).collect::<Vec<Group>>();

        for group in adjacent_groups {
            self.try_capture(group)
        }

        let own_group = self.get_containing_group(point);

        self.try_capture(own_group);

        Ok(())
    }

    fn get_adjacent_points(&self, point: Point) -> Set<Point> {
        let (x, y) = point;
        let Dimensions { x: board_x, y: board_y } = self.dimensions;

        Set::from_iter(
            [
                (x != 0).then_some((x - 1, y)),
                (y != 0).then_some((x, y - 1)),
                (x != board_x - 1).then_some((x + 1, y)),
                (y != board_y - 1).then_some((x, y + 1)),
            ]
            .into_iter()
            .flatten(),
        )
    }

    fn get_containing_group(&self, point: Point) -> Group {
        let mut group = Group::new();

        self.add_and_recurse(&mut group, point);

        group
    }

    fn add_and_recurse(&self, group: &mut Group, point: Point) {
        let point_color = self.board[point].stone;

        if group.contains(&point) || point_color.is_none() {
            return;
        }

        group.insert(point);

        for adjacent_point in self.get_adjacent_points(point) {
            let adjacent_point_color = self.board[adjacent_point].stone;
            if adjacent_point_color == point_color {
                self.add_and_recurse(group, adjacent_point);
            }
        }
    }

    fn try_capture(&mut self, group: Group) {
        if self.is_alive(&group) {
            return;
        };

        log::trace(&format!("Capture | Group: {group:?}"));

        for point in group {
            if let Some(color) = self.board[point].stone {
                self.captures.insert(!color, self.captures[&!color] + 1);
                self.board[point].stone = None;
            }
        }
    }

    fn is_alive(&self, group: &Group) -> bool {
        let group_and_bordering = group.iter().flat_map(|point: &Point| self.get_adjacent_points(*point)).collect::<Group>();
        let bordering = &group_and_bordering - group;

        bordering.into_iter().any(|point: Point| self.board[point].stone.is_none())
    }
}
