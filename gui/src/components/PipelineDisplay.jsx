import React, { useState } from "react";

import styled from "styled-components";

import Table from "react-bootstrap/Table";

import { Badge } from "./styled";
import CheckInput from "./CheckInput";

const PipelineDiv = styled.div`
margin-left: 1rem;
margin-right: 1rem;
`;

const CurrentRecentOnlyCheckInput = styled(CheckInput)`
margin-top: 1rem;
margin-bottom: 1rem;
`;

const NoPipelineBadgeContainer = styled.div`
text-align: center;
`;

const RECENT_ONLY_KEY = "pipelineRecentOnly";

const PipelineDisplay = (props) => {
    let storedCurrentRecentOnly = localStorage.getItem(RECENT_ONLY_KEY) ||
						   "true";
    const [recentOnly, setRecentOnly] = useState(
	   storedCurrentRecentOnly === "true");

    var pipelines = props.pipelines;

    if (recentOnly === true && props.pipelines.length > 0) {
	   pipelines = [props.pipelines[0]];
    }

    const onCurrentRecentOnlyChange = () => {
	   localStorage.setItem(RECENT_ONLY_KEY, !recentOnly);
	   setRecentOnly(!recentOnly);
    };

    return (
	   <PipelineDiv>
		  <h3>Pipeline</h3>

		  <CurrentRecentOnlyCheckInput
			 value={recentOnly}
			 onClick={onCurrentRecentOnlyChange}
			 label="Only Show Most Recent" />

		  <Table bordered>
			 <thead>
				<tr>
				    <th>Step #</th>
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
					   <td>{recentOnly === true ?
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
