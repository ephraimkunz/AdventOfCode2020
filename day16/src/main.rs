use std::ops::RangeInclusive;

#[derive(Debug)]
struct TicketField {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
    final_indices: Vec<usize>,
    final_index: Option<usize>,
}

#[derive(Debug)]
struct Ticket {
    values: Vec<usize>,
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let mut groups = input.split("\n\n");
    let mut field_defs = groups
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let colon_index = s.find(':').unwrap();
            let or_index = s.find(" or ").unwrap();
            let first_dash = s.find('-').unwrap();
            let last_dash = s.rfind('-').unwrap();

            let name = &s[..colon_index];

            let mut ranges = vec![];
            let first_range = RangeInclusive::new(
                (&s[(colon_index + 2)..first_dash])
                    .parse::<usize>()
                    .unwrap(),
                (&s[(first_dash + 1)..or_index]).parse::<usize>().unwrap(),
            );
            ranges.push(first_range);

            let second_range: RangeInclusive<usize> = RangeInclusive::new(
                (&s[(or_index + 4)..last_dash]).parse::<usize>().unwrap(),
                (&s[(last_dash + 1)..]).parse::<usize>().unwrap(),
            );
            ranges.push(second_range);

            TicketField {
                name: name.to_string(),
                ranges,
                final_indices: vec![],
                final_index: None,
            }
        })
        .collect::<Vec<_>>();

    let my_ticket = groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| Ticket {
            values: s.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
        })
        .collect::<Vec<_>>()
        .into_iter()
        .next()
        .unwrap();

    let nearby = groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| Ticket {
            values: s.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
        })
        .collect::<Vec<_>>();

    part1(&field_defs, &nearby);
    part2(&mut field_defs, &nearby, &my_ticket);
}

fn part1(defs: &[TicketField], nearby: &[Ticket]) {
    let mut invalid_values = 0;

    for ticket in nearby {
        for num in &ticket.values {
            if !defs
                .iter()
                .flat_map(|d| &d.ranges)
                .any(|r| r.contains(&num))
            {
                invalid_values += num;
            }
        }
    }

    println!("{}", invalid_values);
}

fn part2(defs: &mut [TicketField], nearby: &[Ticket], my_ticket: &Ticket) {
    let filtered_tickets = nearby
        .iter()
        .filter(|ticket| {
            for num in &ticket.values {
                if !defs
                    .iter()
                    .flat_map(|d| &d.ranges)
                    .any(|r| r.contains(&num))
                {
                    return false;
                }
            }
            return true;
        })
        .collect::<Vec<_>>();

    let num_fields = defs.len();
    for def in defs.iter_mut() {
        for candidate_index in 0..num_fields {
            let all_match = filtered_tickets.iter().all(|t| {
                let matches1 = def.ranges[0].contains(&t.values[candidate_index]);
                let matches2 = def.ranges[1].contains(&t.values[candidate_index]);
                matches1 || matches2
            });
            if all_match {
                def.final_indices.push(candidate_index);
            }
        }
    }

    for _ in 0..num_fields {
        let single_candidate = defs
            .iter_mut()
            .find(|d| d.final_indices.len() == 1)
            .unwrap();

        let index = *single_candidate.final_indices.first().unwrap();
        single_candidate.final_index = Some(index);
        for def in defs.iter_mut() {
            if let Some(pos) = def.final_indices.iter().position(|x| *x == index) {
                def.final_indices.remove(pos);
            }
        }
    }

    let mut answer = 1;
    for field in defs.iter().filter(|f| f.name.starts_with("departure")) {
        answer *= my_ticket.values[field.final_index.unwrap()];
    }

    println!("{}", answer);
}
