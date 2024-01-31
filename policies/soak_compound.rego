package xchemlab.soak_compound

import data.xchemlab
import future.keywords.if

default read_well = {"allowed" : false}
default write_well = {"allowed" : false}
default read_compound = {"allowed" : false}
default write_compound = {"allowed" : false}
default read_soaked = {"allowed" : false}
default write_soaked = {"allowed" : false}

read_well = response if {
    xchemlab.valid_token
    response := {
        "allowed": true, 
        "subject": xchemlab.subject,
    }
}

write_well = response if {
    xchemlab.valid_token
    response := {
        "allowed": true, 
        "subject": xchemlab.subject,
    }
}

read_compound = response if {
    xchemlab.valid_token
    response := {
        "allowed": true, 
        "subject": xchemlab.subject,
    }
}

write_compound = response if {
    xchemlab.valid_token
    response := {
        "allowed": true, 
        "subject": xchemlab.subject,
    }
}


read_soaked = response if {
    xchemlab.valid_token
    response := {
        "allowed": true, 
        "subject": xchemlab.subject,
    }
}

write_soaked = response if {
    xchemlab.valid_token
    response := {
        "allowed": true, 
        "subject": xchemlab.subject,
    }
}