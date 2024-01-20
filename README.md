## Python wrapper for RISC Zero prover

<img src="title.png" align="right" alt="many military tanks rolling in parallel on the desert" width="350"/>

When people talk about accelerating zero-knowledge proofs, there are usually two approaches:
- hardware acceleration
- distributed computation

After years of exploration, many in the industry (including Supranational, Ingonyama) would agree that Nvidia GPU and 
Apple Metal GPU seems to be doing pretty well for hardware acceleration. FPGA and ASIC are still too early to compete, 
and evidence in chip design suggests that FPGA/ASIC are unlikely to beat GPU---any idea that can challenge this assertion 
is most welcomed. In fact, [Omer Shlomovits](https://www.omershlomovits.com/) from [Ingo](https://www.ingonyama.com/) 
and I have a bounty for breakthrough ideas in hardware acceleration.

This leaves distributed computing. 

The idea is that, if we have a zero-knowledge proof task, we want to distribute it to multiple machines and then aggregate 
their work together. This would require a zero-knowledge proof system that is _**distributed-computation-friendly**_.
- the different machines involving in the process are **laconic** and **taciturn**, i.e., they have **minimalistic** communication between each other.
- the individual proof work can be merged in an efficient way without severely sacrificing the overall proof generation time

This idea has, however, been systematically studied. 

### Background in Ray


### Cost efficiency of distributed computing