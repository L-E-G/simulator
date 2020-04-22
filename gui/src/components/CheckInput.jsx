import React from "react";

import styled from "styled-components";

import checkedIcon from "../images/checked.png";
import uncheckedIcon from "../images/unchecked.png";

const CheckImg = styled.img`
width: 1.5rem;
margin-right: 1rem;
`;

const CheckInput = (props) => {
    let value = props.value;
    let onClick = props.onClick;
    let label = props.label;

    const imgSrc = value === true ? checkedIcon : uncheckedIcon;

    return (
	   <div onClick={onClick}>
		  <CheckImg src={imgSrc} />
		  {label}
	   </div>
    );
};

export default CheckInput;
