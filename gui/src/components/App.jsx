import React, { useState } from "react";

import styled from "styled-components";

import Navbar from "react-bootstrap/Navbar";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";

import { Simulator } from "simulator";

import logoIcon from "../images/logo.png";
import stepIcon from "../images/step.png";

import { colors } from "../styles";
import { SecondaryButton } from "./styled";

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

const StepButton = styled(SecondaryButton)`
float: right;
`;

const StepImg = styled.img`
width: 1.5rem;
margin-right: 0.5rem;    
`;

const regAddrAliases = {
    26: "INTLR",
    27: "IHDLR",
    28: "PC",
    29: "STATUS",
    30: "SP",
    31: "LR",
};

/**
 * Wraps Simulator class methods so that the React state is updated when the
 * internal simulator state changes.
 */
class GUISimulator {
    /**
     * @param {Simulator} simulator - Base simulator instance
	* @param {Object} stateSetters - React hook state setters, keys are: 
	*     setRegisters, setDRAM, setPipeline.
     */
    constructor(simulator, stateSetters) {
	   this.simulator = simulator;
	   
	   this.setRegisters = stateSetters.setRegisters;
	   this.setDRAM = stateSetters.setDRAM;
	   this.setPipeline = stateSetters.setPipeline;
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
	   this.simulator.step();
	   
	   this.setRegisters(this.simulator.get_registers());
	   this.setDRAM(this.simulator.get_dram());
	   this.setPipeline(this.simulator.get_pipeline());
    }
}

var simulator = new Simulator();

const App = () => {
    const [registers, setRegisters] = useState(simulator.get_registers());
    const [dram, setDRAM] = useState(simulator.get_dram());
    const [pipeline, setPipeline] = useState(simulator.get_pipeline());
    const [error, setError] = useState(null);

    var guiSimulator = new GUISimulator(simulator, { setRegisters,setDRAM,
										   setPipeline });

    const onStepClick = () => {
	   try {
		  guiSimulator.step();
	   } catch (e) {
		  setError(e);
	   }
    };

    return (
	   <div>
		  <ErrorContext.Provider value={[error, setError]}>
			 <SimulatorContext.Provider value={guiSimulator}>
				<AppNavbar expand="md">
				    <Navbar.Brand>
					   <BrandImg src={logoIcon} alt="LEG computer logo" />
					   <BrandName>LEG Simulator</BrandName>
				    </Navbar.Brand>

				    <Navbar.Collapse className="justify-content-end">
					   <Navbar.Text>
						  <StepButton onClick={onStepClick}>
							 <StepImg src={stepIcon} />
							 Step
						  </StepButton>
					   </Navbar.Text>
				    </Navbar.Collapse>
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
