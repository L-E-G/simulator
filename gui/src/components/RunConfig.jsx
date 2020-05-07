import React, { useContext } from "react";
import styled from "styled-components";

import ToggleCard from "./ToggleCard";
import CheckInput from "./CheckInput";

import { SimulatorContext, PROG_STATUS_NOT_RUNNING } from "./App";

const RunConfigCard = styled(ToggleCard)`
max-width: 18rem;
`;

const ConfigCheckInput = styled(CheckInput)`
margin-top: 1rem;
`;

const RunConfig = (props) => {
    const guiSimulator = useContext(SimulatorContext);

    const runConfig = props.runConfig;
    const programStatus = props.programStatus;

    var _props = {...props};
    delete _props.runConfig;
    delete _props.programStatus;

    const notRunning = programStatus === PROG_STATUS_NOT_RUNNING;

    const onPipelineEnabledClick = () => {
	   guiSimulator.set_run_config({
		  ...runConfig,
		  pipeline_enabled: !runConfig.pipeline_enabled,
	   })
    };

    return (
	   <RunConfigCard {..._props} title="Run Configuration">
		  {notRunning === false &&
		  <i>
			 Cannot change the run configuration after the simulator
			 has started.
		  </i>}
		  
		  <ConfigCheckInput
			 value={runConfig.pipeline_enabled}
			 onClick={onPipelineEnabledClick}
			 label="Use Pipeline"
			 disabled={notRunning === false}
		  />
	   </RunConfigCard>
    );
};

export default RunConfig;
