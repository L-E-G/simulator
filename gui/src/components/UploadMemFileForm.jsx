import React, { useState, useContext, useEffect } from "react";

import styled from "styled-components";

import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Form from "react-bootstrap/Form";
import Spinner from "react-bootstrap/Spinner";

import { SimulatorContext, ErrorContext } from "./App.jsx";
import CheckInput from "./CheckInput";
import { PrimaryButton } from "./styled";
import ToggleCard from "./ToggleCard";

import { colors } from "../styles";

const UploadCard = styled(ToggleCard)`
width: 32rem;
`;

const ExampleSelectCol = styled(Col)`
border-left: 0.1rem solid rgba(0,0,0,.1);
`;

const UploadFileInput = styled(Form.File)`
margin-top: 0.9rem;

& > .custom-file-input {
    cursor: pointer;
}

& > .custom-file-label {
    text-align: center;
}

& > .custom-file-label::after {
    display: none;
}
`;

const InitialFileInput = styled(UploadFileInput)`
& > .custom-file-label {
    background: ${colors.primary};
    color: white;
}
`;

const ReuploadFileInput = styled(UploadFileInput)`
& > .custom-file-label {
    background: ${colors.red};
}
`;

const UseExampleButton = styled(PrimaryButton)`
margin-top: 1rem;
`;

const LoadSameFileCheck = styled(CheckInput)`
margin-top: 1rem;
`;

const SHOULD_USE_MEM_FILE_KEY = "shouldUseMemoryFile";
const STORED_MEM_FILE_KEY = "memoryFile";

const DRAM_EX_SIMPLE_LOAD = "simple-load";

var DRAM_EXAMPLE_FILES = {}
DRAM_EXAMPLE_FILES[DRAM_EX_SIMPLE_LOAD] = [0,0,128,32];

const UploadMemFileForm = (props) => {
    const simulator = useContext(SimulatorContext);
    const setError = useContext(ErrorContext)[1];

    const [fileLoading, setFileLoading] = useState(false);
    const [fileSelected, setFileSelected] = useState(false);
    const [useSameFile, setUseSameFile] = useState(
	   localStorage.getItem(SHOULD_USE_MEM_FILE_KEY) === "true");
    const [useSameFileCheck, setUseSameFileCheck] = useState(useSameFile);
    const [exampleFile, setExampleFile] = useState(DRAM_EX_SIMPLE_LOAD);

    useEffect(() => {
	   // Load DRAM if use same file option is set
	   if (useSameFile === true && fileSelected === false) {
		  let storedMemFileItem = localStorage.getItem(STORED_MEM_FILE_KEY);

		  if (storedMemFileItem !== null) {
			 try {
				var arr = [];
				let split = storedMemFileItem.split(",");
				for (var i in split) {
				    arr.push(Number(split[i]));
				}

				simulator.set_dram(new Uint8Array(arr));
				
				setFileSelected(true);
			 } catch (e) {
				setError(e);
			 }
		  }
	   }
    }, [fileSelected, setError, simulator, useSameFile]);
    
    var reader = new FileReader();

    reader.onload = () => {
	   try {
		  var array = new Uint8Array(reader.result);
		  simulator.set_dram(array);

		  if (useSameFile === true) {
			 localStorage.setItem(STORED_MEM_FILE_KEY, array);
		  }

		  setFileLoading(false);
	   } catch(e) {
		  setError(e);
		  setFileSelected(false);
		  setFileLoading(false);
	   }
    };
    
    const onFileChange = (e) => {
	   setFileSelected(true);
	   setFileLoading(true);
	   setUseSameFile(useSameFileCheck);
	   
	   reader.readAsArrayBuffer(e.target.files[0]);
    };

    const onUseSameFileChange = () => {
	   localStorage.setItem(SHOULD_USE_MEM_FILE_KEY, !useSameFileCheck);
	   setUseSameFileCheck(!useSameFileCheck);
    };

    const onExampleSubmit = () => {
	   setFileLoading(true);
	   
	   try {
		  var array = new Uint8Array(DRAM_EXAMPLE_FILES[exampleFile]);
		  simulator.set_dram(array);
		  console.log("set dram", exampleFile, DRAM_EXAMPLE_FILES, array);

		  if (useSameFile === true) {
			 localStorage.setItem(STORED_MEM_FILE_KEY, array);
		  }

		  setFileLoading(false);
	   } catch(e) {
		  setError(e);
		  setFileSelected(false);
		  setFileLoading(false);
	   }
    };

    const onExampleFileChanged = (e) => {
	   setExampleFile(e.target.value);
    };

    const ExampleSelectEl = (
	   <div>
		  <Form.Group controlId="exampleFileSelect">
			 <Form.Label>Example Memory File</Form.Label>
			 <Form.Control
				value={exampleFile}
				onChange={onExampleFileChanged}
				as="select"
			 >
				<option value={DRAM_EX_SIMPLE_LOAD}>Simple Load</option>
			 </Form.Control>

			 <UseExampleButton
				onClick={onExampleSubmit}
				type="submit"
			 >
				Use Example
			 </UseExampleButton>
		  </Form.Group>
	   </div>
    );

    const LoadSameFileCheckEl = (
	   <LoadSameFileCheck
		  value={useSameFileCheck}
		  onClick={onUseSameFileChange}
		  label="Load the same file in the future" />
    );

    const FormContents = () => {
	   if (fileLoading === false) {
		  return (
			 <React.Fragment>
				<Container>
				    <Row>
					   <Col>
						  Upload a file to set the contents of
						  simulator memory.
						  
						  {fileSelected === true ?
						   <ReuploadFileInput
							  label="Load Another"
							  onChange={onFileChange}
							  custom />
						  :
						   <InitialFileInput
							  label="Select File"
							  onChange={onFileChange}
							  custom />
						  }
					   </Col>

					   <ExampleSelectCol>
						  {ExampleSelectEl}
					   </ExampleSelectCol>
				    </Row>
				</Container>

				<hr />

				{LoadSameFileCheckEl}
			 </React.Fragment>
		  );
	   } else {
		  return (
			 <div>
				<Spinner animation="grow" role="status" />
				<span className="sr-only">Loading...</span>
			 </div>
		  );
	   }
    };

    return (
	   <UploadCard
		  title="Memory File"
		  startExpanded={fileSelected === false}
	   >
		  <FormContents />
	   </UploadCard>
    );
};

export default UploadMemFileForm;
export { SHOULD_USE_MEM_FILE_KEY, STORED_MEM_FILE_KEY };
