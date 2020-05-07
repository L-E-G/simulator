import React, { useState } from "react";
import styled from "styled-components";

import Card from "react-bootstrap/Card";

import ToggleExpandButton from "./ToggleExpandButton";

const StyledCard = styled(Card)`
margin: 1rem;

& > .card-body {
    padding-bottom: 0.5rem;
}
`;

const CardTitle = styled(Card.Title)`
display: flex;
`;

const CardTitleText = styled.span`
flex-grow: 1;
`;

const Children = styled.div`
padding-bottom: 1rem;
`;

const ToggleCard = (props) => {
    const title = props.title;
    const children = props.children;
    const startExpanded = props.startExpanded !== undefined ?
					 props.startExpanded : true;

    const [toggleUsed, setToggleUsed] = useState(false);
    const [expanded, setExpanded] = useState(true);
    if (toggleUsed === false && expanded !== startExpanded) {
	   setExpanded(startExpanded);
    }

    var _props = {...props};
    delete _props.title;
    delete _props.children;
    delete _props.startExpanded;

    const doToggleExpand = () => {
	   setToggleUsed(true);
	   setExpanded(!expanded);
    };

    return (
	   <StyledCard {..._props}>
		  <Card.Body>
			 <CardTitle>
				<CardTitleText>{title}</CardTitleText>

				<ToggleExpandButton
				    expanded={expanded}
				    doToggleExpand={doToggleExpand}/>
			 </CardTitle>

			 {expanded &&
			  <Children>{children}</Children>}
		  </Card.Body>
	   </StyledCard>
    );
};

export default ToggleCard;
