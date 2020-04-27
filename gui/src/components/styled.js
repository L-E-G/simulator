import styled from "styled-components";

import Button from "react-bootstrap/Button";
import BootstrapBadge from "react-bootstrap/Badge";
import Dropdown from "react-bootstrap/Dropdown";

import { colors } from "../styles";

const Color = require("color");

const SecondaryButton = styled(Button)`
background: ${colors.secondary};

&.active, &:hover {
    background: white;
    color: ${colors.primary};
}

&:disabled {
    background: ${Color(colors.secondary).darken(0.5)};
    border: 1px solid ${Color(colors.secondary).darken(0.3)};
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
color: white;
`;

const DropdownToggle = styled(Dropdown.Toggle)`
&.dropdown-toggle {
    border: 1px solid ${colors.primary};
    background: none;
    color: ${colors.primary};
}

.show > &, &:hover {
    background: ${colors.primary} !important;
}
`;

export { SecondaryButton, OutlinedButton, Badge, DropdownToggle };
