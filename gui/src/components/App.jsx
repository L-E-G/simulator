import React, { useState } from "react";

import styled from "styled-components";

import Navbar from "react-bootstrap/Navbar";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Spinner from "react-bootstrap/Spinner";

import { Simulator } from "simulator";

import logoIcon from "../images/logo.png";
import stepIcon from "../images/step.png";
import sleepIcon from "../images/sleep.png";
import runningIcon from "../images/running.png";
import happyIcon from "../images/happy.png";
import playIcon from "../images/play.png";
import pauseIcon from "../images/pause.png";

import { colors } from "../styles";
import { Badge, SecondaryButton } from "./styled";

import MemoryTable from "./MemoryTable.jsx";
import UploadMemFileForm from "./UploadMemFileForm.jsx";
import PipelineDisplay from "./PipelineDisplay.jsx";
import Error from "./Error";

const SimulatorContext = React.createContext(null);
const ErrorContext = React.createContext([{}, () => {}]);

const AppNavbar = styled(Navbar)`
background: ${colors.primary};
`;

const BrandImg = styled.img`
width: 2.5rem;
transition-duration: 1s;

&:hover {
  transform: rotate(360deg);
}
`;

const BrandName = styled.span`
margin-left: 10px;
color: white;
`;

const StatusBadge = styled(Badge)`
padding: 0.7rem;
margin-right: 1rem;
font-size: 1rem;
font-weight: normal;
`;

const ControlButton = styled(SecondaryButton)`
background: none;
color: ${colors.primary};
border: 0;
`;

const ControlButtonImg = styled.img`
width: 1.5rem;
margin-right: 0.5rem;    
`;

const SimulatorStatusesContainer = styled.div`
display: flex;
justify-content: center;
`;

const SimulatorStatuses = styled.div`
display: flex;
border-radius: 0.25rem;
border: none;
background: ${colors.secondary};
`;

const SimulatorStatusBadge = styled.div`
border-left: 0.1rem solid ${colors.primary};
display: flex;
padding: 0.5rem;
color: white;
align-items: center;

&:first-of-type {
    border-left: none;
}

&.has-button {
    padding: 0;
    transition: background 0.15s;
}

&.has-button button {
    border-radius: 0;
    color: white;
}

&.has-button button:hover {
    background: white;
    color: ${colors.primary};
    border-radius: 0.25rem;
}

&.has-button:hover {
    background: white;
}
`;

const regAddrAliases = {
    26: "INTLR",
    27: "IHDLR",
    28: "PC",
    29: "STATUS",
    30: "SP",
    31: "LR",
};

const PROG_STATUS_COMPLETED = "Completed";
const PROG_STATUS_NOT_RUNNING = "Not Running";
const PROG_STATUS_RUNNING = "Running";

/**
 * Wraps Simulator class methods so that the React state is updated when the
 * internal simulator state changes.
 */
class GUISimulator {
    /**
     * @param {Simulator} simulator - Base simulator instance
	* @param {Object} stateSetters - React hook state setters, keys are: 
	*     setRegisters, setDRAM, setPipeline, setCycleCount, setProgramStatus.
     */
    constructor(simulator, stateSetters) {
	   this.simulator = simulator;
	   
	   this.setRegisters = stateSetters.setRegisters;
	   this.setDRAM = stateSetters.setDRAM;
	   this.setPipeline = stateSetters.setPipeline;
	   this.setCycleCount = stateSetters.setCycleCount;
	   this.setProgramStatus = stateSetters.setProgramStatus;
    }

    set_registers(v) {
	   this.simulator.set_registers(v);
	   this.setRegisters(this.simulator.get_registers());
    }
    
    set_dram(v) {
	   this.simulator.set_dram(v);
	   this.setDRAM(this.simulator.get_dram());
    }

    set_pipeline(v) {
	   this.simulator.set_pipeline(v);
	   this.setPipeline(this.simulator.get_pipeline());
    }

    step() {
	   let keepRunning = this.simulator.step();

	   if (keepRunning === true) {
		  this.setProgramStatus(PROG_STATUS_RUNNING);
	   } else {
		  this.setProgramStatus(PROG_STATUS_COMPLETED);
	   }
	   
	   this.setRegisters(this.simulator.get_registers());
	   this.setDRAM(this.simulator.get_dram());
	   this.setPipeline(this.simulator.get_pipeline());
	   this.setCycleCount(this.simulator.get_cycle_count());
    }
}

var simulator = new Simulator();

const App = () => {
    const [registers, setRegisters] = useState(simulator.get_registers());
    const [dram, setDRAM] = useState(simulator.get_dram());
    const [pipeline, setPipeline] = useState(simulator.get_pipeline());
    const [cycleCount, setCycleCount] = useState(simulator.get_cycle_count());
    const [programStatus, setProgramStatus] = useState(PROG_STATUS_NOT_RUNNING);
    const [programPlaying, setProgramPlaying] = useState(false);
    const [error, setError] = useState(null);

    var guiSimulator = new GUISimulator(simulator, { setRegisters,
										   setDRAM,
										   setPipeline,
										   setCycleCount,
										   setProgramStatus });

    const onStepClick = () => {
	   try {
		  guiSimulator.step();
	   } catch (e) {
		  setError(e);
	   }
    };

    if (programStatus !== PROG_STATUS_COMPLETED && programPlaying === true) {
	   onStepClick();
    }

    const onRunClick = () => {
	   setProgramPlaying(!programPlaying);
    };

    var programStatusImg = sleepIcon;

    switch (programStatus) {
	   case PROG_STATUS_RUNNING:
		  programStatusImg = runningIcon;
		  break;
	   case PROG_STATUS_COMPLETED:
		  programStatusImg = happyIcon;
		  break;
    }

    return (
	   <div>
		  <ErrorContext.Provider value={[error, setError]}>
			 <SimulatorContext.Provider value={guiSimulator}>
				<AppNavbar expand="md">
				    <Navbar.Brand>
					   <BrandImg src={logoIcon} alt="LEG computer logo" />
					   <BrandName>LEG Simulator</BrandName>
				    </Navbar.Brand>

				    <Navbar.Text>
					   <SimulatorStatusesContainer>
						  <SimulatorStatuses>
							 <SimulatorStatusBadge>
								<ControlButtonImg src={programStatusImg} />
								
								<b>{programStatus}</b>
							 </SimulatorStatusBadge>
							 
							 <SimulatorStatusBadge>
								<b>Program Counter</b>: {registers[28]}
							 </SimulatorStatusBadge>
							 
							 <SimulatorStatusBadge>
								<b>Cycle Count</b>: {cycleCount}
							 </SimulatorStatusBadge>

							 <SimulatorStatusBadge className="has-button">
								<ControlButton
								    onClick={onRunClick}
								    disabled={programStatus === PROG_STATUS_COMPLETED ? true : null}>
								    <ControlButtonImg src={programStatus === PROG_STATUS_RUNNING ? pauseIcon : playIcon} />
								    {programStatus === PROG_STATUS_RUNNING ? "Pause" : "Play"}
								</ControlButton>
							 </SimulatorStatusBadge>

							 <SimulatorStatusBadge className="has-button">
								<ControlButton
								    onClick={onStepClick}
								    disabled={programStatus === PROG_STATUS_COMPLETED  ? true : null}>
								    <ControlButtonImg src={stepIcon} />
								    Step
								</ControlButton>
							 </SimulatorStatusBadge>
						  </SimulatorStatuses>
					   </SimulatorStatusesContainer>
				    </Navbar.Text>
				</AppNavbar>

				<Error />

				<UploadMemFileForm />

				<PipelineDisplay pipeline={pipeline} />

				<Container fluid>
				    <Row>
					   <Col>
						  <MemoryTable
							 title="Registers"
						      keyAliases={regAddrAliases}
							 memory={registers} />
					   </Col>
					   <Col>
						  <MemoryTable title="DRAM" memory={dram} />
					   </Col>
				    </Row>
				</Container>
			 </SimulatorContext.Provider>
		  </ErrorContext.Provider>
	   </div>
    );
};

export default App;
export { SimulatorContext, ErrorContext };
