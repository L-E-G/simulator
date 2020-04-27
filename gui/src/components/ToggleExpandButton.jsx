import React from "react";

import styled from "styled-components";

import { OutlinedButton } from "./styled";

const ToggleButton = styled(OutlinedButton)`
display: inline-block;
width: 27px;
height: 24px;
line-height: 24px;
padding: 0;
padding-bottom: 1.6rem;
`;

const ToggleExpandButton = (props) => {
    let expanded = props.expanded;
    let doToggleExpand = props.doToggleExpand;
    
    var txt = "▲";

    if (!expanded) {
	   txt = "▼";
    }
    
    return (
	   <ToggleButton
		  className={props.className}
		  variant="outline-primary"
		  onClick={doToggleExpand}>
		  {txt}
	   </ToggleButton>
    );
};

export default ToggleExpandButton;
