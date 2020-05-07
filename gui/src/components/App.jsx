import React, { useState } from "react";

import styled from "styled-components";

import Navbar from "react-bootstrap/Navbar";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";

import { Simulator } from "simulator";

import logoIcon from "../images/logo.png";
import stepIcon from "../images/step.png";
import sleepIcon from "../images/sleep.png";
import runningIcon from "../images/running.png";
import happyIcon from "../images/happy.png";
import playIcon from "../images/play.png";

import { colors } from "../styles";
import { SecondaryButton } from "./styled";

import MemoryTable from "./MemoryTable.jsx";
import UploadMemFileForm from "./UploadMemFileForm.jsx";
import PipelineDisplay from "./PipelineDisplay.jsx";
import RunConfig from "./RunConfig";
import AssemblerInput from "./AssemblerInput";
import Help from "./Help";
import Error from "./Error";

const SimulatorContext = React.createContext(null);
const ErrorContext = React.createContext([{}, () => {}]);

const AppContainer = styled.div`
margin-top: 5rem;
`;

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

const MemoryContainer = styled(Container)`
margin-top: 2rem;
`;

const PC_REG_IDX = 28;

var regAddrAliases = {
    26: "INTLR",
    27: "IHDLR",
    29: "STATUS",
    30: "SP",
    31: "LR",
};
regAddrAliases[PC_REG_IDX] = "PC";

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
	*     setRunConfig, setRegisters, setCache, setDRAM, setPipelines,
	*     setCycleCount, setProgramStatus.
     */
    constructor(simulator, stateSetters) {
	   this.simulator = simulator;

	   this.setRunConfig = stateSetters.setRunConfig;
	   this.setRegisters = stateSetters.setRegisters;
	   this.setCache = stateSetters.setCache;
	   this.setDRAM = stateSetters.setDRAM;
	   this.setPipelines = stateSetters.setPipelines;
	   this.setCycleCount = stateSetters.setCycleCount;
	   this.setProgramStatus = stateSetters.setProgramStatus;
    }

    set_dram_assembled(v) {
	   this.simulator.set_dram_assembled(v);
	   this.setDRAM(this.simulator.get_dram());
    }

    set_run_config(c) {
	   this.simulator.set_run_config(c);
	   this.setRunConfig(this.simulator.get_run_config());
    }

    set_registers(v) {
	   this.simulator.set_registers(v);
	   this.setRegisters(this.simulator.get_registers());
    }
    
    set_dram(v) {
	   this.simulator.set_dram(v);
	   this.setDRAM(this.simulator.get_dram());
    }

    step() {
	   let keepRunning = this.simulator.step();

	   if (keepRunning === true) {
		  this.setProgramStatus(PROG_STATUS_RUNNING);
	   } else {
		  this.setProgramStatus(PROG_STATUS_COMPLETED);
	   }
	   
	   this.setRegisters(this.simulator.get_registers());
	   this.setCache(this.simulator.get_cache());
	   this.setDRAM(this.simulator.get_dram());
	   this.setPipelines(this.simulator.get_pipelines());
	   this.setCycleCount(this.simulator.get_cycle_count());
    }

    finish_program() {
	   this.simulator.finish_program();

	   this.setRegisters(this.simulator.get_registers());
	   this.setCache(this.simulator.get_cache());
	   this.setDRAM(this.simulator.get_dram());
	   this.setPipelines(this.simulator.get_pipelines());
	   this.setCycleCount(this.simulator.get_cycle_count());

	   this.setProgramStatus(PROG_STATUS_COMPLETED);
    }
}

var simulator = new Simulator();

const App = () => {
    const [runConfig, setRunConfig] = useState(simulator.get_run_config());
    const [registers, setRegisters] = useState(simulator.get_registers());
    const [cache, setCache] = useState(simulator.get_cache());
    const [dram, setDRAM] = useState(simulator.get_dram());
    const [pipelines, setPipelines] = useState(simulator.get_pipelines());
    const [cycleCount, setCycleCount] = useState(simulator.get_cycle_count());
    const [programStatus, setProgramStatus] = useState(PROG_STATUS_NOT_RUNNING);
    const [error, setError] = useState(null);

    var guiSimulator = new GUISimulator(simulator, { setRunConfig,
										   setRegisters,
										   setCache,
										   setDRAM,
										   setPipelines,
										   setCycleCount,
										   setProgramStatus });

    const onStepClick = () => {
	   try {
		  guiSimulator.step();
	   } catch (e) {
		  setError(e);
	   }
    };

    const onRunClick = () => {
	   try {
		  guiSimulator.finish_program();
	   } catch (e) {
		  setError(e);
	   }
    };

    var programStatusImg = null;

    switch (programStatus) {
	   case PROG_STATUS_RUNNING:
		  programStatusImg = runningIcon;
		  break;
	   case PROG_STATUS_COMPLETED:
		  programStatusImg = happyIcon;
		  break;
	   default:
		  programStatusImg = sleepIcon;
		  break;
    }

    return (
	   <AppContainer>
		  <ErrorContext.Provider value={[error, setError]}>
			 <SimulatorContext.Provider value={guiSimulator}>
				<AppNavbar expand="md" fixed="top">
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
								<b>Program Counter</b>
								: {registers[PC_REG_IDX]}
							 </SimulatorStatusBadge>
							 
							 <SimulatorStatusBadge>
								<b>Cycle Count</b>: {cycleCount}
							 </SimulatorStatusBadge>

							 <SimulatorStatusBadge className="has-button">
								<ControlButton
								    onClick={onRunClick}
								    disabled={programStatus === 
									   PROG_STATUS_COMPLETED ? 
										    true : null}>
								    <ControlButtonImg src={playIcon} />

								    Run
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

				<Help />

				<Container fluid>
				    <Row>
					   <Col>
						  <UploadMemFileForm />
					   </Col>

					   <Col>
						  <RunConfig
							 programStatus={programStatus}
							 runConfig={runConfig}
						  />
					   </Col>

					   <Col>
						  <AssemblerInput />
					   </Col>
				    </Row>
				</Container>

				<PipelineDisplay
				    pipelineStatuses={pipelines}
				    runConfig={runConfig}
				/>

				<MemoryContainer fluid>
				    <Row>
					   <Col>
						  <MemoryTable
							 title="Registers"
						      keyAliases={regAddrAliases}
							 memory={registers} />
					   </Col>
					   <Col>
						  <MemoryTable title="DRAM" memory={dram} />
						  <MemoryTable title="Cache" memory={cache} />
					   </Col>
				    </Row>
				</MemoryContainer>
			 </SimulatorContext.Provider>
		  </ErrorContext.Provider>
	   </AppContainer>
    );
};

export default App;
export { SimulatorContext, ErrorContext,
	    PROG_STATUS_COMPLETED, PROG_STATUS_NOT_RUNNING, PROG_STATUS_RUNNING };
