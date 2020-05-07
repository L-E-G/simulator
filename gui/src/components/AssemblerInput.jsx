import React, { useState, useContext } from "react";
import styled from "styled-components";

import Form from "react-bootstrap/Form";
import Spinner from "react-bootstrap/Spinner";

import ToggleCard from "./ToggleCard";
import { PrimaryButton } from "./styled";
import { ErrorContext, SimulatorContext } from "./App";
import CheckInput from "./CheckInput";
import { SHOULD_USE_MEM_FILE_KEY, STORED_MEM_FILE_KEY } from "./UploadMemFileForm";

const LoadSameCheck = styled(CheckInput)`
margin-top: 1rem;
`;

const SubmitButton = styled(PrimaryButton)`
margin-top: 1rem;
`;

const AssemblerInput = () => {
    const guiSimulator = useContext(SimulatorContext);
    const setError = useContext(ErrorContext)[1];
    
    const [assembleText, setAssembleText] = useState("");
    const [loading, setLoading] = useState(false);
    const [loadSame, setLoadSame] = useState(
	   localStorage.getItem(SHOULD_USE_MEM_FILE_KEY) === "true");

    const onAssembleTextChange = (e) => {
	   setAssembleText(e.target.value);
    };

    const onLoadSameClick = () => {
	   setLoadSame(!loadSame);
    }

    const onSubmitClick = () => {
	   setLoading(true);

	   try {
		  guiSimulator.set_dram_assembled(assembleText);

		  if (loadSame === true) {
			 localStorage.setItem(SHOULD_USE_MEM_FILE_KEY, true);
			 let dram = guiSimulator.simulator.get_dram();
			 var dramArr = new Uint32Array();

			 for (var i in dram) {
				dramArr.push(dram[i]);
			 }
			 
			 localStorage.setItem(STORED_MEM_FILE_KEY, dramArr);
		  }
	   } catch (e) {
		  setError(e);
	   }
	   
	   setLoading(false);
    };

    /*
    		  <LoadSameCheck
			 label="Load same assembly in the future"
		      onClick={onLoadSameClick}
			 value={loadSame}
		  />
		  */
    
    return (
	   <ToggleCard
		  title="Program Assembler"
	   >
		  <Form.Control
			 as="textarea"
			 value={assembleText}
			 onChange={onAssembleTextChange}
		  />



		  <SubmitButton onClick={onSubmitClick}>
			 {loading === false ?
			  "Assemble and Load" :
			  <React.Fragment>
				 <Spinner animation="border" />
				 Assembling
			  </React.Fragment>
			 }
		  </SubmitButton>
	   </ToggleCard>
    );
};

export default AssemblerInput;
