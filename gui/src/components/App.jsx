import React, { useState, useContext, useEffect } from "react";

import styled from "styled-components";

import Navbar from "react-bootstrap/Navbar";
import Button from "react-bootstrap/Button";

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

const App = () => {
    var simulator = new Simulator();

    const [registers, setRegisters] = useState(simulator.get_registers());
    const [dram, setDRAM] = useState(simulator.get_dram());
    const [pipeline, setPipeline] = useState(simulator.get_pipeline());
    const [error, setError] = useState(null);
    
    const onStepClick = () => {
	   try {
		  simulator.step();

		  setRegisters(simulator.get_registers());
		  setDRAM(simulator.get_dram());
		  setPipeline(simulator.get_pipeline());
	   } catch (e) {
		  setError(e);
	   }
    };
    
    return (
	   <div>
		  <ErrorContext.Provider value={[error, setError]}>
			 <SimulatorContext.Provider value={simulator}>
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

				<UploadMemFileForm setDRAM={setDRAM} />

				<PipelineDisplay pipeline={pipeline} />

				<MemoryTable title="Registers" memory={registers} />
				<MemoryTable title="DRAM" memory={dram} />
			 </SimulatorContext.Provider>
		  </ErrorContext.Provider>
	   </div>
    );
};

export default App;
export { SimulatorContext, ErrorContext };
