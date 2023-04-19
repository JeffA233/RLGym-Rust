# RLGym-Rust <br />

This is still somewhat WIP and undocumented. <br />

TL;DR this is a Rust remake of RLGym. The primary intended purpose for my own project is to use it as a python module with PyO3/Maturin to have less compute cost vs dealing with Python. If there is more interest in this then I may clean this up more and try to write some real documentation but I expect there to be very little interest. I do not have very rigorous error handling with Rocket League crashing yet, consider that a warning if Rocket League tends to crash for you with regular RLGym. <br />

There are a couple changes done relative to regular RLGym. The first one is observation functions are put into a Vec for the possibility to have agents of multiple types of agents if desired. The second one is that terminal conditions should only return a single boolean for the sake of allowing complete explicit control of the boolean although in the end it really likely does not matter in terms of functionality. <br />

I have included my lib.rs as an example of how I use the gym with Python. I have also included my custom_conditions.rs to show the slight change that should be done compared to normal RLGym. You can find the reward functions for examples as they would be in any of the reward function files in regular RLGym. <br />

# --TODO LIST--
✅-AdvancedObs (but not default obs yet, though it's easy to implement) <br />
~-Better exception handling <br />
✅-General action parsers like Discrete and Continuous (Default is not technically implemented though) <br />
~-Various checks to make sure all functionality match between Python and Rust <br />
