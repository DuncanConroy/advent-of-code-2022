fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let marker = get_start_of_packet_marker(input.as_bytes());
    println!("First packet marker after: {marker}");

    let marker = get_start_of_message_marker(input.as_bytes());
    println!("First message marker after: {marker}");
}

fn get_start_of_packet_marker(input_stream: &[u8]) -> usize {
    let mut unique_tokens_found = 0u32;
    for (index, byte) in input_stream.iter().enumerate() {
        unique_tokens_found <<= 8;
        unique_tokens_found |= *byte as u32;
        let as_bits = format!("{:#X}", unique_tokens_found);
        if index > 3 {
            if ((unique_tokens_found & 0xFF000000) >> 24) != *byte as u32
                && ((unique_tokens_found & 0x00FF0000) >> 16) != *byte as u32
                && ((unique_tokens_found & 0x0000FF00) >> 8) != *byte as u32
                && ((unique_tokens_found & 0xFF000000) >> 24) != ((unique_tokens_found & 0x0000FF00) >> 8)
                && ((unique_tokens_found & 0x00FF0000) >> 16) != ((unique_tokens_found & 0x0000FF00) >> 8)
                && ((unique_tokens_found & 0xFF000000) >> 24) != ((unique_tokens_found & 0x00FF0000) >> 16)
            {
                return index + 1;
            }
        }
    }
    0
}

fn get_start_of_message_marker(input_stream: &[u8]) -> usize {
    let mut unique_tokens_found = 0u128;
    for (index, byte) in input_stream.iter().enumerate() {
        unique_tokens_found <<= 8;
        unique_tokens_found |= *byte as u128;
        let as_bits = format!("{:#X}", unique_tokens_found);
        if index > 3 {
            if ((unique_tokens_found & 0xFF00000000000000000000000000) >> 24) != *byte as u128
                && ((unique_tokens_found & 0x00FF0000) >> 16) != *byte as u128
                && ((unique_tokens_found & 0x0000FF00) >> 8) != *byte as u128
                && ((unique_tokens_found & 0xFF000000) >> 24) != ((unique_tokens_found & 0x0000FF00) >> 8)
                && ((unique_tokens_found & 0x00FF0000) >> 16) != ((unique_tokens_found & 0x0000FF00) >> 8)
                && ((unique_tokens_found & 0xFF000000) >> 24) != ((unique_tokens_found & 0x00FF0000) >> 16)
            {
                return index + 1;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, vec};
    use crate::{get_start_of_message_marker, get_start_of_packet_marker};

    #[test]
    fn gets_packet_markers() {
        // given: input streams and expectations
        let input = vec![("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
                         ("nppdvjthqldpwncqszvftbrmjlhg", 6),
                         ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
                         ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)];

        // when: the get_marker function is invoked
        let result = input.iter().map(|it| get_start_of_packet_marker(it.0.as_bytes())).collect::<Vec<usize>>();

        // then: the first marker with x (configurable) unique tokens is returned
        assert_eq!(result, input.iter().map(|it| it.1).collect::<Vec<usize>>());
    }

    #[test]
    fn get_message_markers() {
        // given: input streams and expectations
        let input = vec![("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
                         ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
                         ("nppdvjthqldpwncqszvftbrmjlhg", 23),
                         ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
                         ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        // when: the get_marker function is invoked
        let result = input.iter().map(|it| get_start_of_message_marker(it.0.as_bytes())).collect::<Vec<usize>>();

        // then: the first marker with x (configurable) unique tokens is returned
        assert_eq!(result, input.iter().map(|it| it.1).collect::<Vec<usize>>());
    }
}
