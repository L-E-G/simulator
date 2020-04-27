import React, { useState, useContext, useEffect } from "react";

import styled from "styled-components";

import Form from "react-bootstrap/Form";
import Card from "react-bootstrap/Card";
import Spinner from "react-bootstrap/Spinner";

import { SimulatorContext, ErrorContext } from "./App.jsx";
import ToggleExpandButton from "./ToggleExpandButton.jsx";
import CheckInput from "./CheckInput";

import { colors } from "../styles";

const UploadCard = styled(Card)`
width: 15rem;
margin: 1rem;
`;

const UploadToggleButton = styled(ToggleExpandButton)`
float: right;
`;

const UploadCardTitle = styled(Card.Title)`
margin-bottom: 0;
`;

const UploadFileInput = styled(Form.File)`
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

const LoadSameFileCheck = styled(CheckInput)`
margin-top: 1rem;
`;

const SHOULD_USE_MEM_FILE_KEY = "shouldUseMemoryFile";
const STORED_MEM_FILE_KEY = "memoryFile";

const UploadMemFileForm = (props) => {
    const simulator = useContext(SimulatorContext);
    const setError = useContext(ErrorContext)[1];

    const [expanded, setExpanded] = useState(true);
    const [fileLoading, setFileLoading] = useState(false);
    const [fileSelected, setFileSelected] = useState(false);
    const [useSameFile, setUseSameFile] = useState(
	   localStorage.getItem(SHOULD_USE_MEM_FILE_KEY) === "true");
    const [useSameFileCheck, setUseSameFileCheck] = useState(useSameFile);

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
				
				setExpanded(false);
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
		  setExpanded(false);
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

    const LoadSameFileCheckEl = (
	   <LoadSameFileCheck
	   value={useSameFileCheck}
	   onClick={onUseSameFileChange}
	   label="Load the same file in the future" />
    );

    const FormContents = () => {
	   if (!fileSelected) {
		  return (
			 <div>
				<InitialFileInput
				    label="Select File"
				    onChange={onFileChange}
				    custom />

				{LoadSameFileCheckEl}
			 </div>
		  );
	   } else {
		  if (fileLoading) {
			 return (
				<div>
				    <Spinner animation="grow" role="status" />
				    <span className="sr-only">Loading...</span>
				</div>
			 );
		  } else {
			 return (
				<div>
				    <ReuploadFileInput
					   label="Load Another"
					   onChange={onFileChange}
				        custom />

				    {LoadSameFileCheckEl}
				</div>
			 );
		  }
	   }
    };

    const doToggleExpand = () => {
	   setExpanded(!expanded);
    };
    
    return (
	   <UploadCard>
		  <Card.Body>
			 <UploadCardTitle>
				<span>Memory File</span>

				<UploadToggleButton
				    expanded={expanded}
				    doToggleExpand={doToggleExpand}/>
			 </UploadCardTitle>

			 {expanded &&
				<div>
				    <Card.Body>
					   Upload a file to set the contents of simulator memory.
				    </Card.Body>
				    
				    <form>
					   <FormContents />
				    </form>
				</div>
			 }
		  </Card.Body>
	   </UploadCard>
    );
};

export default UploadMemFileForm;
