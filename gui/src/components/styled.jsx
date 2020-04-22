import React from "react";

import styled from "styled-components";

import Button from "react-bootstrap/Button";
import BootstrapBadge from "react-bootstrap/Badge";

import { colors } from "../styles";

const SecondaryButton = styled(Button)`
background: ${colors.secondary};

&.active, &:hover {
    background: white;
    color: ${colors.primary};
}
`;

const OutlinedButton = styled(Button)`
background: white;
color: ${colors.primary};
border: 1px solid ${colors.primary};

&.active, &:hover {
    background: ${colors.primary} !important;
    colors: white;
}
`;

const Badge = styled(BootstrapBadge)`
background: ${colors.secondary};
`;

export { SecondaryButton, OutlinedButton, Badge };
