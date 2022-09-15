# RLGym-Rust
# --TODO LIST--
Behold a weird table thing:

RLGym    
│  ├──gym    
│  ├──make    
│  └──version    
│    
├──communication    
│  ├──communication_exception_handler    
│  ├──communication_handler    
│  └──message    
│    
├──envs    
│  ├──environment    
│  └──match    
│    
├──gamelaunch    
│  ├──epic_launch    
│  ├──launch    
│  ├──minimize    
│  └──paging    
│    
└──utils    
   ├──action parsers    
   │  ├──action_parser    
   │  ├──continuous_act    
   │  ├──default_act    
   │  └──discrete_act    
   │    
   ├──gamestates    
   │  ├──game_state ✓   
   │  ├──physics_object ✓   
   │  └──player_data ✓   
   │    
   ├──obs_builder    
   │  ├──advanced_obs    
   │  ├──default_obs    
   │  ├──obs_builder    
   │  └──rhobot_obs    
   │    
   ├──reward_functions    
   │  ├──common_rewards    
   │  │  ├──ball_goal_rewards    
   │  │  ├──conditional_rewards    
   │  │  ├──misc_rewards    
   │  │  └──player_ball_rewards    
   │  │    
   │  ├──combined_reward    
   │  ├──default_reward    
   │  └──reward_function    
   │    
   ├──state_setters    
   │  ├──wrappers    
   │  │  ├──car_wrapper    
   │  │  ├──physics_wrapper    
   │  │  └──state_wrapper    
   │  │    
   │  ├──default_state    
   │  ├──random_state    
   │  ├──state_setter    
   │  └──state_wrapper    
   │      
   ├──terminal_conditions    
   │  ├──common_conditions    
   │  └──terminal_condition    
   │    
   ├──common_values    
   └──math ✓    
