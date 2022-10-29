use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

struct FrameWidget {

}

impl Widget for FrameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}