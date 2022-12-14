pub fn part_1() -> u32 {
    let rucksacks_contents: Vec<&str> = INPUT.split("\n").collect();
    let rucksacks_commons: Vec<char> = rucksacks_contents.iter().map(|rucksack_contents| {
        let (first_compartment, second_compartment) = rucksack_contents.split_at(rucksack_contents.len() / 2);
        let commons: Vec<char> = first_compartment.chars().filter(|&x| second_compartment.contains(x)).collect();

        commons[0]
    }).collect();

    sum_of_priorities(rucksacks_commons)
}

pub fn part_2() -> u32 {
    let rucksacks_contents: Vec<&str> = INPUT.split("\n").collect();
    let rucksacks_groups: Vec<Vec<&str>> = rucksacks_contents.chunks(3).map(|chunk| chunk.into()).collect();
    let groups_commons: Vec<char> = rucksacks_groups.iter().map(|rucksack_group| {
        let commons: Vec<char> = rucksack_group[0].chars().filter(|&x| rucksack_group[1].contains(x) && rucksack_group[2].contains(x)).collect();

        commons[0]
    }).collect();

    sum_of_priorities(groups_commons)
}

fn sum_of_priorities(commons: Vec<char>) -> u32 {
    let groups_commons_priorities: Vec<u32> = commons.iter().map(|&x| {
        if x.is_lowercase() {
            x as u32 - 96
        } else {
            x as u32 - 64 + 26
        }
    }).collect();

    groups_commons_priorities.iter().sum()
}

static INPUT: &str = "\
mjpsHcssDzLTzMsz
tFhbtClRVtbhRCGBFntNTrLhqrwqWMDMTWTqMq
LltbngLGRSBgSgGRCJdSdQHvdfmQccmjSQ
lBslsZDDWdGdGpSMts
grQhDvqLQHDNGJJtbRMQQJ
HChCTnnLCgCrTZPPFzzVPcVD
ShrzjhNGrNqrhWnHHfVHbhnHbbhH
RBsvcBcDCdsRTsvgCgcPFRQpVQGQJPVFbnJfbJ
DvsTsdlCBsGLrjzmlqqz
WJJqZTgCnBLGCZBJCJnTLggTDDSDDMNdDSdbdSSsWDFfMsFf
PVjqpVHmPpvmcjhrRprFmQQffbfNbQMMsSMQNQ
cwcpRvrVlVgwtBwZqBzZ
qfJJmpqpmhsggvvpVPZCrhdFLFzZFDdLLh
CtCTBctGcGLSzZddGZSW
RlNjBCnjttBHHMMcQHCsRfsbfwgggmmJvmgfpm
ZmcgBBZhZMsnqnCPjpHPjLHp
dGbNwNtlTMTzGfNvTvdwNGVLPpQHPjLQPCpCjPqjLbpLPR
dvDTdfvNBhDZMBDZ
cvvRvbqcllbBVlvVVbVVlbVDjRjDjdMsHPZPGdDPGPHrDP
FwtpfwJtWwNtTTNnwFCtjDJsQdQPPPPMrjrPJHjH
CwFpnppgntShgbsscbms
cWMFMQpFNcvNDdBDgdsT
MPrrfrCHBBsDZCBJ
LmLjMLjjLWpVcRVR
ZrRZqlZMqTWrMDqwvnvVtnsvddvVnlVf
pQNhhLNNGmLjhhcfvndDpffdfdVf
QGjCLCQGmNgPBQDFFgTMJWWwMRTrTZWWBWTr
WrZWZPHHWZHprZVmVvqddBttBBhGhtvh
gzDlMTJDMfqhBGllhl
jJLqMMDDbbqjLpPHcsHLWZspPr
bsSVRVGsrDstrrSjcQjcjlPwzjQl
gHBggFNTTvTgfqgCFzljWwLWQQQnrwQWnf
NvJHgpgHvqBhNBJhHTvpBCJCZmtdpDsGsZdZMZRbVbbMdrZs
MPPtPwPnRnMPPnwrtNSGgLSCGGGNSLtSgD
hBhWFjfCsTbbbWqFFWBBqBhsWZVGSVglZHLSVDlNWDNHHGgV
zsCfTsTCMdmRPwzQ
JVQVvvszzvTsVsVJjctppcCtjtPRcTlP
MdFgqSddMqMDbtDlNjRDSR
qFZWZqwHlZfZvzvZfLZn
vpqwQSsHSHDQzDpgzwZlRLRZRRZTnTrrvGhh
JBcdmbmFMPgPbgfrZRZnRFFnrnLRln
JNdBNgbdJmPMWSSDzwVDtwSWWW
BDMcDDppHCStpWcHBDNtzPJjqGlllPMJzPGjwjlq
CZdZLmgCdqbPzjblZj
vndLfnghRQmVrhdvgBHpSCDWHBBCVHNppD
WrhrJJGSWzpTWwts
VlLPmqgmRNZRGwsvttjgcwsT
PDZmlbdVqLmPlddVNRDmmmbbSFHrCFQCnFBFJHSJGrDQCBrr
hvPdpvhHvHvPrNfVhDfjggFfRV
zlGwJGslsSDRfjsg
MJMWjMJzwqWGzJwMqJBTCmHndPPdCBvmdCpmHn
PVWFpQhJhFJpGbRCvRHGCp
jgslDjftsqhNglTgllgTqMnlHwCcvwZwRccSRCbGSGbCMHRw
TgjhNNnjlTfjTdDqTfhjnmzmWPzWrLdrQBPJFWJWBB
qPPRMPlfSzSSSPPnnLnqMlpQQtrrtmWpbFtQrdzrtrWt
BBvCcwsVThsBgswDBCFQHQpdmQvtrFpWFvWp
gCghTJgVCgDGVMlRGMqZnSWqlM
RWbHvrbHBsbWBHJWvJwMtmdZwdtmdvwMZQff
DRVjcqhRchhGGllhCgdGQQzfttzGQGwQfg
cDRljchpqTcjDFTFVcPcPCWBHpNnJNNSnbWbHHrSpWHr
dtHrRrBHrCRhddftjgBrRhgjsbbbMpbSWSTjWcsDTWDbcW
GQPFVQVQnJlqVMDcMzpDfzpDVD
qZZJFLlLnvFFGPGLPqnJvwQldfgHrBRBmBhgNBRHghNhhwRg
rLbrZhPgqZhMdVFSFTSGCqFG
zsszfRzjtHtzvRTSDdFFCtdDdtND
fcwllfmwzRHlfmmzFvQQLrgLMLBZhJQZPrZhJLhW
sllrCfpQQJpMHLgzwDwpNqzzVDpV
RZPFZPGcSMFtGPRGMwNDVwdRgzvNwgqNvg
hBmbMcBmcThmcGtSFTZfQCJjrHLJfsjhWJssJl
DqGCbGfCRhfZCVbbqDJJGJBgRNpNdpBNNgNBBNwHnRgt
rcWSsSSPSQtwBwHD
MLscLMzvvTvcTLzvWWFDPTTrGqmFGGqCZJGbblbVbVZZVmFJ
FprpsLQTrstQHNmVSVml
JMggWPggWcRbwgJPCGMcGcfmzHlMNSjfzVNhHfVtzSMz
cwnPnBwgnGRgRCgRbWJLpFsLtFBLFrDLFZZDrL
lVgjLLLMgFMDCwCFqCRbngsvnGSvnSGndbsfgf
WZJcTWcNTmJZphmTJJNQHcdvfdbvnRRGbGthdrbttfSv
ZPQTJTpTNPJNQTmJRBZJNBHjwMVwPCMwVlVzjwwzqqjVjL
hznNhNQNQFDWVFmDQm
SMqZBMMbBvDbHPzzdVPH
zzzTBTMLNTgpnTTh
NLCdmsdCVLGHCHdQzzmznnFwRjFMDMwpTBjDRpnpTBMJ
PrcfcrglcfWbSqgrlqvShrwpJpDBFJHpBWjTDTRTRTTB
crSgSHtPttfdLGmtzzZNNV
BTlTVqCBqtTcBqVhWlsJjDvsnLsvlvpJPj
gMgggGZbSMzNRRRLmZZnQZQPPDvnsnDvJwwQ
dMRRmMgbNfRgmfSdGFgNgTBtrhrhqfWtLCCWLTWWHc
zcfVrPwnwrPmrvnjdFdBbHFFdd
CCqpSSQQpQZLDCSHPpBFvFBjTHRvRR
DMLGthLZMLtQGhGNMPqGSDflzfwcVmzJzsfgNVrswcrr
hSgvMTQvChSqPvhTrRLlVHJgfgRJlHHHJH
jmzsZzZzwmmLGGtwtVJWNNDRDtVcfVRl
GnBBLbzzzFszBFpzvSdrQQCTCQbhMvSQ
VHpTMrZMMbDbbpTZmQmTnmzhTqjqlWWQ
GGvgNsvNCNvvGvlqqdzWZmlsmZqZ
wNNNgccNGJSNBSRNBNvNcvJHLDDZMFRMppMLrfHDLbrrHF
spssbPMLpPllspGNsNWMrnwddnfcqrnwwwwMwM
VmQBFCjzzjmfnwbrngcVrd
FQbSFjBvvzsWvWGlvWNl
JLFSwfwRLLfGhnQJBQshvn
pZgNcpCWpWtcvhjGGjtVvszD
CccMcPcgTTCWmcZcWMcmTNZPmHdrqSHFRRrqwrSrRqwrHmsH
BPMhflJRhqnPNGjNRNRjgSRm
VdVsDswTVZbCwCZBrcDCczTwtjtNNjmjmgpmjpQggpGVSgQm
sTbWrsTBbrTPPnqlJnPPhW
nvrgjMWBvQWPvQnsZfGcZcRFdGFtdtZB
bHVDwmqNNDhHNzqpphLNHVLpSJcdZtfffRZdDgRFGSddcRZt
HNLNqNqLNbhqVVbClngjnQWPTWgsCgvT
tfstpcScscBTFTpFnsWSmgdzJlgmgBmPPzJmvdPm
jnrqrLHRwGrwhdPvvPvhjJmP
qqCLRCGrZZqCHRVtVWQptFWppnbcWb
wCDJZJgDwHpdqHhdGHBhhH
WSPmJMlmbSmztQlQsvPhnhGGdBddBqdGddTbVB
WzWQftWMSWtmvmmSWtMQPgggpZwLwZjggJFgrpFCfj
MvQBJMBQhjQFNFnjnj
dtlZmRtLmjSTSLLtTtNVwWzDRzDVwwWFwnNn
dmmLCqTdcLqtLGqjBhpfHqBGpv
PBPRhjTPPlLRBvlvfwffqJGfpG
rHtMtrszFtSgbFrrggrFgMnwWGzmQqWvGWzGQpJGfNqqNz
FggcbSMntVgMdRCwZcjChLCT
lCqqBlCwlnDqPZTZZBLNdjJLwttNWjjdzJzc
fVfMbvbvmbVsmSsmMVWNtzzcjgLWgjztMMtg
VVmFhFRSfbQsvVQmvSfhSsmzHlCZqrrBrDBrHZPRTZnnzB
CRrDWmzRRQMmDqrrBgBQmtHljhHwtwlwplcBjHGwwB
PWfPSWnvsNZSZdfjHjZtGHjchllltl
WVsnbSPTbNdbmqTQmmrmLzTq
cGtMBGSJDgtgMBsBMgMvWWSHWjpjzHTWTPpqWzqW
mNVQNsdVsdhLmCpTWWjmCjTT
NQQwrfbQrNQNbrrdLwfQsZdgFbBBFBgggRGRDMRFFMRDgM
lFnqgqWQvHWqgvlVglvqjPjcLdfLfBPLnrbLNLcN
hmTmthppsRtpTRRTZMpSbLdNjNcJLcrcBNbJBZZc
smmpRsTtpSSsRGhppmmhdCMGWwqFQgWGWWDgWwVFHQqHgg
mWFjmcdcFWcSSQjzrpvrwRGvTwQGGG
HRJfgMZVhtRlHJHBVJTGvGppbpbvvGTvTtrv
glsgVMVqffdnPRDcqLnL
MtvLJdmLLTvSSCtSzLSTcDhRjRftQjjssshfQNjPtf
nlggrFWzRsfFjVQN
WgwwBgbgZBHGBnccTzMCLTZJmLLL
sRtHTBBHZtDTtZhdPzWdGcdVFdJmGcnm
wpwMLWCgvfNvwvwbbCrwgfzPncrJPSFVGPnrcJSVznmV
bLpvwQwMwpjWMgfvgZTsDsBttqHRjTqHlH
mpmGpCpmlpmwfmCQVppCVfQSSjvSqgWvvvDgNwWDgnnDnW
RBLsHRJBRrHJWFDWSNqFWj
zZBLdsdcZrsBjGfpGVpTTPGlVc
NBbTzgwSNmrFWpVrzrFM
LnZQtQlZVnMrFBBG
CCdtddBtPdNqcvHSCCcg
ZFbZPHbZPTQVVlsGNF
qtvDWvgRftqGNccCNVThDs
fRwGBBjBppdMdBMZ
GffflsZsPZVfjsssNfZsJNNZVcMDSqMWFcwFMMpcTMTTFSTS
LhrCmvzcRbbhtmRdTCMDwWMpDWqqqMpW
dvRQmBBvLzBRRvRhhcdbhdRgjHQNllJsfsNlZZljZGGNQN
wjbMPsbfLzVCTMVbjLplmpshhSpHShhJhtsm
ZrcqZTDTGDqFdJtGmdGSpl
QNNrWvQRqRNWnTQRvqjPbjfWbCBCMbMLBwMV
wRPRsppFfWJRlPRPFlpJfwSMzzZTBwBtZTTCMCMtdz
vGLGrjcfrLVGjfnGTMCMtNNnCTnMtCBd
VrjqhjhLVcrGVRqJmqQspmfFWm
LRfdnmwMwdSBmfvJNrrgLhCNgqqJWs
llctPPVTcPStgJgshCsrCs
DpTlFpFVRFZRFFSv
sPgRgsmdcqmgSvvFRRRRdqdFfTWZhhdZrZbbWfTpwDfbWTbw
jLCCHtLljJzjlplfZSlwTfprZZ
tBHVjQHzHQJBtSVmvRsvvFRqnGgv
spppVDbVcbgVSFgFZZbGZgbJMRBTvHTvJJHGtHRwtMGvHT
LldflzQLLQmQWQQfnwMWwHJtTtwRBcBt
CPjfhCmNmNfFVchpchFVhp
bZQJgQmQmTgnLBRtNPNnml
ccszcqldGzhszrVsqdlHVNwLpppwHPHRtBBppDNRLt
VSzVhVdcfrrhcqGrVhrssQQlMbJvFjMgbFSQggCvCv
hHWVWhhlZDZVWNTgczWLjbtcTFFj
JJnPnCdBCBnnRCjSsjStBgsbFttb
MRpgCpGqdPRppJwpnRqRfZZhmvhHDrhllDHhhZGZ
SPcgLDcLLnWFWCNVCRPT
fhZQtsbtmbmfZTVTVRWfNvTCTT
jhbbmzRsQzpLDcgLHLjg
GSFRHrCCGRJDJtrgWdrL
stcVQshQZBsBmjMsZhmMQQWDDvNWdncNWvzLgdDnzdDN
sVwMBQBhVVjtQZVPlSfPCfwRpSCpRl
bBHHJMJvBvWMJWqqccNNPhMCrclChQCC
RPppPgfpwgmcQgrhmm
tfwTwpFPGGwZSRtpVjJHbHLvSvLSqVLL
jlJfZGjljJPBqJGnfGVMqGfrFWWddvDmFRDcmdFDdDvbDM
hTCTsgsgwhTbvRdcFmsddpFd
wbQNHTQLgCwSThhCgwnZnJfqnqJBlNlBnnVl
CLlfbjjbLlbbDGbLzfCGhdtdWBthdBWsHvWHBnntWs
rmJRJFqrDwVFTwFmSJvtvMtdJMMHBdBBndWt
ZVrVVZpgTpZFSqmZqRNlNNfQQbpGjDQbbpPl
mVCrhGHGmZhrNlDwbWnLWWvGLWWwnd
PNsqgzspsgNFJNFfzqpWSWdwSvSPnvdWbSbvjd
NzgJzqMcgscQqJcpJRzBmlrBRBDDlHZBBBHtHZ
NJmNJDwcMmJNMbJJDNDqcGcsWRWHQzRPQjZLRGZWLQsjZQ
dgSnTBgdpddtgShSTZjLRhRLHqWPPhPRPQ
VgdTpBntlvBVrlfcbqJcMrfmcqmb
wvqwvPwNJgFmLdvDJFDmDLvJlQZpMzSpBVflpdSSMlQnfldS
WjCcRZCWRjjRtsZhRRhpSVBVnzplBfWnfBfSQz
CbRbcsjHZrhbTRtsGbCrgNgDDPFFqvvJvJFDFw
GlsCrbCChShqgqlbSCcVbqgVhBwjBDFBhBhdDWvwBFFvWvDv
THmHMmtMnLfHRnzRZnfLBDWWsWzWFNsvWjjvjvFF
mpHRtmZffHTTMpmLMLLnJtJCgScScsPlblcpCrPbblCPlq
vscDLrcvrsLNStdTfBCvgJTqGBdd
bwLbzRhbbdTfbgCB
pplQzLwmPZVMStcDjFtQrS
RMjCrhFJhRVRVCCFFsvmnvqrmbvqmqSmbrvm
tzfpBgTHzttGzZpBfHGDBZHbccnGqbmvdNlGnSnlcvSwbn
pDWTHDTzgTfWZpVVsWSPjRFSMsFs
fmrfmrwVfjmrzjqCsqqvjsvvpG
hFDVtFStVtJnPPtJNHbtQWGbQsCvCsQgpWGggdQC
NBSDSNHStHNHnhStHNNrcflrmTzBlwmzrlMVVw
SjtZZSdNcDldPQqndl
BbgzgWgTmTBfwrbnDjQDwVPwDlnsVq
zBBrCTTMBWLMWmfMfbbmrMtjNZLFJRRZSSvFFtStvGGJ
CTCGLGCFRRSMGnZnLCTfdffhpbNbDfpdZBvhdv
rJlqclVPHJWVrgPPQqjqgJlhBhDBBQdvbhwvNfhswfNpvb
tltrcrHjlVWVCDzSGCCzLCGt
sbHHsbCCHbLSVfJbbfSLNJBzvzMMPrhPPNztZlZNZhdt
GTWjplTgDnGmQGpQnQhZrvvBMPztPzvrzvjZ
mQgGWllcFcTFmgwcDppDQGTCqfsSLqsfSbqJqLSSFsbRfF
jslsFjLLLLvFwWtQFTFDJQWp
dGzdrNmRWqVBGcTbwpRDRnbJDRhT
qzqzrrPNNrmfLPsjglHjgW
QjCHcPfcgQSgPPcffQSmmmLmrJJpNpBMrJMtFrBBBMFrrpNS
VGVZfDbbVVZWGvDbFrlBZNJBNlNMwwtM
sbvfhqTGTRnhTVGvzgHmgQLQmPqzmPLm
sLwnMHnbnLMjGpZsjGGtpc
ggvJrNNTQgQrNvgqBqZCCjClWjGtWjCpGJFW
TVdrqvVrTNTzBqQQzTRMfHbMwMbZMdMbHwRD
bcfJQQJHsQPCpdpWdPbb
RHjHDwZtrZmRDDtwtjRBVFdWVrrrBClldVphCF
zDgwgNzjmDnMnzMMHncG
vMHRvMhvHWRBRDHhRBwWvRBqLqbGwqnqnnNTbNqdNbbVVr
pslgcZszJltrsZcZgNnnqbSSTSSndbNbzS
cZcgsZgZZgPgmcpfJtfttWBQvmFWjDQDhBmFjDHvFr
bVbBvdTTVLbCgCznLJsJcwHPczfz
NFcDphSDrFjGtZNZjplZGZFnzPHPrzHHzJMnnwfPsPsRJs
cGtGljFmWdvqmVCV
qSNbTvcvTGTvGcgtBNvcbdrdjrnjRnjRVHdDqHrRHj
ZZZZPLWPzPDCCsCRnRdwVFnjdwPVFP
ChlCLLZftfcBvfDv
cRtfctVgmRclmBFGbbMBDDFPtD
svQZhHSHssjTvjpQjSSBBMJMJGDBpPbMzzpGzP
ZsvsCTWhCHhSwwjrwbndldlRnfRNmb
PQdTgdGpRcTccCfj
hHFLHlHBhBlmlDFzHrhhfZNZbfNZcVWNVVZRDjCC
LFLLMHJHSBhBFGGnMMvsGtGGtj
fwmVnVCDVqpNQqqb
ddBcZZWdvGWzBzsWvLvddlNHcHQPbQqqJQNNQHPHQT
WgGvsMMzvgbntDhCmt
JjwhFMmwjJwmCgTgSCSFlPLg
WWbsbVtftBZWtnWtncbQvctTGLpLgCpzPPPlllpzlgPPTQ
TBvnfBffWsfVtTvbZBTNjwjqddhMNqwRMMhdRMrq
SllrbtTSQrSQrbrvvMvzFDsBsssWpWdWbGpGBWNWNW
hhCfmmmjmPLCfmnPLfqPgqqNNBpjZBZQDNQdpWNpdBBsBp
RhLfPhQLQfCRnHfqTHHrFJMrttTwTtzt
BFrFBJMMJnnsNJBFCdLCnmvzbPdCmPnc
LDLVHQRfDvdHdcCmcv
llQDwqSVLwZLZSgsGZMNMgTjTMTr
mrwdbqRhdCNGgZBHbH
jVTPMjvjpvMfTfQfPlpHHZNnNBHgZDGsGMnCsZ
TLlfQpffQvvzhtNqztRFtzcm
DDfvJZZPDHVPSPcSvcgcWCsWQcTTdhQTTh
dMwpbdjRtrFhhTsTFQWqhC
bGRdNpbzlvLfDfZlLZ
bdPQdcpdbpjFqpQcQwqqhhNRhJvWRfrrWBsJrfwN
mMtlZfmtnLZtSnGDlmGWRRhrWLhJsBRvgRWghh
DnMGCtmCzfGMbjdQbVzpqcFH
jwnGggRBvvpBZCljCsCWrhhrsh
FVMcFLqLMqcJfVtDqMJcHMHWCSblzzrWsdhSLlSzbrGCLz
HQVFPDtDQDFFNTZpPgNnGgNn
HNBHNqlqHJQBRNvdmZvmPdZZlpnT
bDbbhDgSfzVVfnvPmfHmTZZd
jgzbwrhVsDgsDWLwJqqBMqcqHL
tzNtJzsJVBHzbjbglCHc
nfmnGnmPhntCgHvtvmCj
MStTwrMTWrTdBZSNLZJNVQ
NVjmwmVGGwGFHstwFHMhTh
psRSzzscZscZpgQQzqQtBBHTTlThHHtTTh
rCprbpZccggcrRzbbRRbscvVVWWvNfvVWnGDCWVNddmd
rphfGDgtPtllrPlFlGrhGjnmnTnjcBsncBBVpTTBmc
SqqZMJCLwgCwJgQRqqgZQNwdBBsBBHVBdTHNsnVNBTccnc
MJqZZMbqgzRCSJZwPtFfGzWhrrfGttWl
cSZqqcwbqVzqCbqVqVZPsvvDCDrffngvphggndhdGh
tTNTMWJNQJHMNGSSprfdGnfdth
WNRHWWMJSRWzswbczsVPRs
HCgcSMhSMBGMdvGf
RNQqbDQqFdRFdmTZfGtPZvtGlQffll
mNpdNrRDbTNrmbpzmpWmbpWcswhcHcjhscSHjSgVHwHn
MwgcFgwMMcscCbMFsMFCgMgPPLWPvptvBvPvtvvWmBBzwG
nhQQjTJRVDdQJrPpmnGGBmvtGvLz
HdJQdJHjrJQDBQjhQVQJhdJcqlFHcSqsNbCbCqCHFqCFgC
JvTnvWtdJLbhJHbMwwHjcGHCwHwQGQ
mqtmsllmfqVFwMwMrrPjmQrC
lfztRZSlRDRVzfdpWnSvWhNdbnpp
rSvrgggzHTNzrHtnptpmlDngZjWj
MdMhqMhsfMSRcGqRsQQRctjjdDnjtjClCjjpZnDlnt
BBMRsQRfRcscGqBfRRsBssPBLLzNLFPwvVFFPTLbbLwHHTvS
pCmCfdPFzmsFsDhFFDsttptpRtJjLnlJRtttHt
ZQwgWZgqJhTTRtgV
GNqWNvcqqQQrMMWcQzDDsSzBDBSssSmhhr";