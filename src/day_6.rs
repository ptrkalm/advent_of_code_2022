pub fn part_1() -> u32 {
    let mut marker_end = 4;
    while !is_start_of_packet(&INPUT[marker_end - 4..marker_end]) {
        marker_end += 1;
    }
    marker_end as u32
}

pub fn part_2() -> u32 {
    let mut marker_end = 14;
    while !is_start_of_packet(&INPUT[marker_end - 14..marker_end]) {
        marker_end += 1;
    }
    marker_end as u32
}

fn is_start_of_packet(marker: &str) -> bool {
    marker.chars().all(|c| marker.chars().filter(|v| c == *v).count() == 1)
}

static INPUT: &str = "vmsvstvtzvzqvzvssvddsgdddrqrsqqwnnffrwrswwztwzzbssgtstvsshrsrrlvrvzrrsqsvqsqbssqrsrjrsrddqnnfqnnrfnrnffjsjnnqddjjbvvbzzshhsffmtftrrpzznlltclltmltmllzclcwwgzzzqvqhqllcrllqdqpqnqbqggcrcsclssrfsssqhhfzzprrjvrjvvblvbllwccztzggspgggqrqcqjcqjcqcmmffmtmltlwwcpplpqqwhwvhvnnqhqsqlqvvgfgqqddlhhstthzhgzgbzbdzbzlzwlzlpljlgjjqlltrrbmrrvvcttsddfjddmbdbffcrfftvvzppncpcvcdvcvvjjjlttcgtcctrrrgmrgmgwwsgscsffswfwqwnnzmzffpwptpntppbddggsbsddmjmssnzsnstsztttjztjjpddwlwrrlnnzsnnfllqrrqmqhhfddtmmpfmfcfgfllngnvnjjmffwbffsqsmsdstdddjssmlssfpprrgzzzwvvftffczfftccwrrdsrsjrssfnsnttqptqtnqtnqtttjqtjqjzjpzpvzvbzvvjqqvcqcwwgwllbwbjjhsjhhchffgdfgffvddrndrdwdwlwqqfqzzntzzhwzzndnjnsjjmvvlqqqqshhmzzjvvcscqqjjhphnppcprrhnnbsspcscbssnsmmznzsnnszsrzrwrswsgwglwlzznpnwpnnjlnlrnntllhvvdrvvprpptqqlzqzrrgfgfhfvhhcscjjfmmjpmjjvwvtvhvqhqvvqvwvpvsschhlmlrlmljjqnqggbwbmwbwgbwwgllqzllrjlrjjfjzffthhpvvsddvjvvzsvvhghlhqqbnqqlnngsngnccnpccmttmrtmmlmccqbbjssbsqbqddnqqdvdvbvgbvvjfjhjzzfhfzffgccdjcjtjpttcrrffvddwgglrgllbttcqtctqtvtwtggrppzvvbzvzgzsznnsqnsqqnwwzrwwnwbwnnhznhzhchwwvwhwlwlnnmwmfmfrmmnvndnffjgjmgmfmhmtmstthjhbjhjddrqrllnflnnwzwjjthjhwhmmflmlfmfddqgdqgqqwbbdsstssrvvbpbgppfwfvvglvvzgvgmvgvzvdvcctddjqddffrprbrggpgpghhwzhzggqrqsrstrssnttrgrsswsrstsdtdzttzvznzjnzzqssjwswcswsgghqghqgqvggtsswwmbbfrrdcctjtjsttrccqbbtcctcscqsssvmmdhmdmvdmvmsvsrvsrvsszvvrpvrpprvppctclcrrchhtmmbddswddglgrrdcdppcbpbgpbggbssnrnbrbllqwqbwbmmcwmmrjrdjjpsjppblbjbsbsgswsmwmgwgmmqpplzlssvcscnngwgvvffvlvtvtddgqgttjljggfzggmfgfqqtrrdbdbbnjnnbjbwwjjqljlnjjvzvhvlhhhsfhsfscsjcjsjbsjjdbdbpbqqsggcnczcchrhjhphlppnwwhthghhfvfrrbzrbrrcsszfmhrjflswthfrlmjbnhblmsldjpnrdgfbcrftnbcltlctbdthhjzsqvbmppcbhctqbtfhdrtrbzjpnzzvtfjtpqgbtmdcnjgtblhmvrpbrcpqfhpzqvbfqhsrjqwqqnggrlgvwndqrhvpzhswnglngwjdgwcnngrvhtcsblpdshqcwfpzcpmrbzqjfpllbbvlcfjtdqchjsgsqgqzsbfnnqcsvpwfmzdhzmcjfjnczldclvpfpntppqbhfqzqdtfwsvdzhphwpzhqqfflswhtldlrmrqpfdlwwszphrwjqvhpqmrzvblvtvsfpgzdhnlmsgpqnqdtbjqccbtnvqmtdggdsvvfwjzbdpnztmsblpgcrmppblmtfnjvjwmhnwqdmdrvbvsvhgbhjwzjfcqzwtvzlfmpzvpwsdsqphmqlzwfrqdjdbsmfdzllqqfhmcmhchjpbqtjbczhbcllmtqgczdfnjsjhhcsfdhwwbzzjdqtlvgfgfwbqztqftfqhsfvbzjcbmtdzszfsgzpqpvtqnmjhlrtbvfphvhwbnqsnwgpznnwcbbvdqvvzllqpjbqhqwtfzlhqgvmrpjmlvczqgmffsjvgzvnvccgnmdmbcqjqvbnzvdgnbdwbszldjwjdsnplgqnjmlvrlzcvlgtrdndhjmrtczqjqrzzzclrdmdmrgvljstqfhldfzgvhcdtmncqjjnghgwdgnhbjztfgpqnvfhzngqcftpbqsnvgrqpczwptghbssljnftzmbrqmnbvcbsshvmmmczjgnbjwltflndtntmfznmqhzjjtmmtwbflhmqcmlmzrnvfqzmzhhszqmmqwgzcnncjwvdzvczsgbpcscjptfpwhvvvstvqzhtptrmbhttcrdzhljpmclrbnzmvmtbmmdhfgpjjwsldsgdjflhmfrgvmgzvntgmlglfsntpjjwgnpwsjvsnbctwlsqvprlfmsqfdmzdsdvbtsnqzflvtlgrmdhfgfmwrcqjvbwqsblnwlmtqfhdwwlchqljbljwzmmqgqpwfnmjlvhmppzsdjlbvqwmwqwqqsrgzmpzzzzstrtrbwttzjcmlssstpntnrwtgthrfmthbjjmmtcvjnwsdpdndccgncjbjdqbwtlbfcmgvmlbrdzzlzcpctjvldbpvvmsrwrhhtdsrwljtmntcftwvppjvtttgslrdvpglzdnhrgnrzjsczsmtcfclhvncrhsfvbppqrbwsmjbhjmllwwlqvhbljrqpzshqvllzqtjfhdlnghwblzjldcmfgfjgcbfwqldvdppjmsqqqmznwtcgpmlpqmdpzctwjpnstvgjsbzspfbvfcrgzzcdbzfthgbjrnjwpprblnhdcfgpdcmljpqqzchslpvlwqjvsqtnvnfjhpvwjnjzghnbslhmrhwdppgtvzzmhzzzsvldphjlhrfjwrctfcmnzpvvwqrczcsmznflcwzsplrsdvmfghmnbbjdzclnzpbtjllcszzjcqbbczqhbnfpcctbmhfglsdtnzdjrjbjqczqflznfwzsrmpcgvgrjmhtpgllcpdnlrzvqljprnpvcglvmbtbzjwqfrhdngwsrfjsqnhfncnvprrcngjzfdvcnphcchbqhqpbbbzgmbfndlffnrpzvplptnrvjlzfczvsbctmdqrzdnpvrtgnsgdjwqshglspdzbhflfbsfvbrqgfzmhzhshdlcnqhzbnhbhmsgffgsmvglhscqsrsvmszjfgdhsglbqgwwjndscsqvpccwhlvjbtvftdppcscjblwtmpvvchpzwmblmbbrrvwlglfqwtgcffnzljnmtpbcrszvblwfqdslpqlrmnvrhvrzwnprnmntzrpsdcnwfgljldpvjwwzzbpcltnvghqvqnvmmrfqsnbstdctqqgtpzqttnrrdstrtlfzmvvbzzwwchrscqlnpzdmphdwdqdwbszlhwsbfscthndrdvhtgbpsmznlfjpwjchvzbmrflcphhgjfwstjnzlllztgzjmnwcglmhrbztzdnbcbrgrlmpprrbthbhslfrsjsrrrqvwmqghcgdvvsmrqdwwbdnzmnsqfflpbbjvzjdgsfjdjmbhptmrnbqpqhcdwtvbdlrdzsrchqlsrccjfbfrnnrctdsqbnjzzvrlcwphsscppdmsbrqrzddrdvjwfrldccmzwrhrbpcqpnvnbgpsrhvbchmrdtwqrlvpwhqgnppmsdvqdldlrjrzntlsfwmwhjsghddsppchqlltrhlwrccvflwjbvptclqvwdmrmghbngbflfspsjmglzcqjdtdtldmmngljwfqvfwnmfdtrsmlqzszhzccgvbnnwtpbgssrcqlqgrsjqfhhwpvdtsclgsntwvcsfpvwfqnmwhmtldfswqrsvzshfzwlnwhqhwsrqqzlsdhvqfwnhjcvmplcljmlhbtqtrpjfjfdfmlhtfpnszptfnjbldscjgvjhpzflhm";