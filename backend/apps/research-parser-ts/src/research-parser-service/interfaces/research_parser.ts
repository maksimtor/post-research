export interface ParseRequest {
    url: string
}

export interface ParseResponse {
    name: string
    authors: Author[]
    affiliations: Affiliation[]
    citations: Citation[]
    fundings: Funding[]
    pubBase: PubBase
    doi: string | undefined
}

export interface Author {
    name: string
    id: string
}

export interface Affiliation {
    name: string
}

export interface Citation {
    name: string
    doi: string
    pubmedId: string | undefined
}

export interface Funding {
    name: string
}

export interface PubBase {
    name: string
}
