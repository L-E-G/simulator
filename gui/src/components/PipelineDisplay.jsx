import React, { useState } from "react";

import styled from "styled-components";

import Table from "react-bootstrap/Table";

import { Badge } from "./styled";
import CheckInput from "./CheckInput";

const PipelineDiv = styled.div`
margin-left: 1rem;
margin-right: 1rem;
`;

const CurrentCycleOnlyCheckInput = styled(CheckInput)`
margin-top: 1rem;
margin-bottom: 1rem;
`;

const NoPipelineBadgeContainer = styled.div`
text-align: center;
`;

const CURRENT_CYCLE_ONLY_KEY = "pipelineCurrentCycleOnly";

const PipelineDisplay = (props) => {
    const [currentCycleOnly, setCurrentCycleOnly] = useState(
	   localStorage.getItem(CURRENT_CYCLE_ONLY_KEY) === "true");

    var pipelines = props.pipelines;

    if (currentCycleOnly === true && props.pipelines.length > 0) {
	   pipelines = [props.pipelines[0]];
    }

    const onCurrentCycleOnlyChange = () => {
	   localStorage.setItem(CURRENT_CYCLE_ONLY_KEY, !currentCycleOnly);
	   setCurrentCycleOnly(!currentCycleOnly);
    };

    return (
	   <PipelineDiv>
		  <h3>Pipeline</h3>

		  <CurrentCycleOnlyCheckInput
			 value={currentCycleOnly}
			 onClick={onCurrentCycleOnlyChange}
			 label="Only Show Current Cycle" />

		  <Table bordered>
			 <thead>
				<tr>
				    <th>Cycle #</th>
				    <th>Fetch</th>
				    <th>Decode</th>
				    <th>Execute</th>
				    <th>Access Memory</th>
				    <th>Write Back</th>
				</tr>
			 </thead>
			 <tbody>
				{pipelines.map((item, i) => (
				    <tr key={i}>
					   <td>{currentCycleOnly === true ?
						   props.pipelines.length - 1 :
						   pipelines.length - i -1 }</td>
					   {Object.keys(item).map(key => (
						  
						  <td key={`pipeline-cycle-${i}-${key}`}>
							 <h3><Badge>
								{item[key]}
							 </Badge></h3>
						  </td>
					   ))}
				    </tr>
				))}
			 </tbody>
		  </Table>

		  {pipelines.length === 0 ?
		   <NoPipelineBadgeContainer>
			  <h3><Badge>No Pipeline Data</Badge></h3>
		   </NoPipelineBadgeContainer> : null}
	   </PipelineDiv>
    );
};

export default PipelineDisplay;
