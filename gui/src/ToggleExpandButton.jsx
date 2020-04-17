import React from "react";

import Button from "react-bootstrap/Button";

import "./ToggleExpandButton.scss";

const ToggleExpandButton = (props) => {
    let expanded = props.expanded;
    let doToggleExpand = props.doToggleExpand;
    
    var txt = "▲";

    if (!expanded) {
	   txt = "▼";
    }
    
    return (
	   <Button id={props.id}
			 className={"toggle-expand-button " + props.className}
			 variant="outline-primary"
			 onClick={doToggleExpand}>
		  {txt}
	   </Button>
    );
};

export default ToggleExpandButton;
