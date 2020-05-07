import React from "react";

import styled from "styled-components";

import checkedIcon from "../images/checked.png";
import uncheckedIcon from "../images/unchecked.png";

const CheckContainer = styled.button`
display: flex;
background: none;
border: none;

&[disabled] > img {
    opacity: 0.5;
}
`;

const CheckImg = styled.img`
width: 1.5rem;
height: 1.5rem;
margin-right: 1rem;
flex-grow: 0;
flex-shrink: 0;
align-self: center;
`;

const CheckInput = (props) => {
    const value = props.value;
    const onClick = props.onClick;
    const label = props.label;

    var _props = {...props};
    delete _props.value;
    delete _props.onClick;
    delete _props.label;
    
    const imgSrc = value === true ? checkedIcon : uncheckedIcon;

    return (
	   <CheckContainer {..._props} onClick={onClick}>
		  <CheckImg src={imgSrc} />
		  <span>{label}</span>
	   </CheckContainer>
    );
};

export default CheckInput;
