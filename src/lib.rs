#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Space {
    pub term_height: u16,
    pub offset: u16,
    pub lines_printed: u16,
    pub frame_height: u16,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Frame {
    pub pad: u16,
    pub shift: u16,
}

impl Space {
    pub fn frame(self) -> Frame {
        let Space {
            term_height,
            offset,
            lines_printed,
            frame_height,
        } = self;

        // The viewport might be clipped, but the math still needs to work out.
        let actual_vh_height = frame_height.clamp(0, term_height);

        // Move the terminal's cursor down to the number of lines printed
        let remaining_space = term_height.saturating_sub(offset + actual_vh_height);

        // Calculate the overlap of the final lines with the frame
        let end_y = offset + lines_printed;
        let frame_cap = term_height - actual_vh_height;
        let term_shift = remaining_space.min(lines_printed);

        let padding = end_y
            .saturating_sub(frame_cap)
            .clamp(0, actual_vh_height - 1);

        Frame {
            pad: padding,
            shift: term_shift,
        }
    }
}

#[cfg(test)]
const BASE_TERM: Space = Space {
    term_height: 16,
    frame_height: 5,
    offset: 0,
    lines_printed: 0,
};

#[test]
fn frame_space_calcs() {
    // Enough space to just write the lines and the frame since we're at the top
    // We do need to shift the frame down by the number of lines printed
    assert_eq!(
        Space {
            lines_printed: 7,
            offset: 0,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 0, shift: 7 }
    );

    // Same, but we're hitting the bottom of the viewport with the final line
    // check for off-by-one errors
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 3,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 0, shift: 8 }
    );

    // Okay, down one more
    // The shift isn't a full 8 anymore, so we need to pushback by 1
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 4,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 1, shift: 7 }
    );

    // Down own more
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 5,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 2, shift: 6 }
    );

    // Down own more
    // ending at line 13 out of 16
    // We need to push the lines up by 3 and then the frame shifts down by
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 6,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 3, shift: 5 }
    );
}

#[test]
fn messy_case() {
    // Down own more
    // ending at line 13 out of 16
    // We need to push the lines up by 3 and then the frame shifts down by
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 7,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 4 }
    );

    // Down own more
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 8,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 3 }
    );

    // Down own more
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 9,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 2 }
    );

    // Down own more
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 10,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 1 }
    );

    // No shift for frame, all padding
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 11,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );

    // Starting very close to the bottom
    // Pad extra with no shift
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 12,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );

    // Starting very close to the bottom
    // Pad extra with no shift
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 13,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );

    // Starting very close to the bottom
    // Pad extra with no shift
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 14,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );

    // Starting very close to the bottom
    // Pad extra with no shift
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 15,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );

    // Starting very close to the bottom
    // Pad extra with no shift
    assert_eq!(
        Space {
            lines_printed: 8,
            offset: 16,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );

    // Starting very close to the bottom
    // Pad extra with no shift
    assert_eq!(
        Space {
            lines_printed: 9,
            offset: 16,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );

    // very long
    assert_eq!(
        Space {
            lines_printed: 29,
            offset: 16,
            ..BASE_TERM
        }
        .frame(),
        Frame { pad: 4, shift: 0 }
    );
}
