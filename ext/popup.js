async function getCurrentTab() {
    let queryOptions = { active: true, lastFocusedWindow: true };
    // `tab` will either be a `tabs.Tab` instance or `undefined`.
    let [tab] = await chrome.tabs.query(queryOptions);
    return tab;
  }

async function fetchData() {
    console.log(getCurrentTab())
    const url = (await getCurrentTab()).url;
    
    // const url = 'https://pubmed.ncbi.nlm.nih.gov/39779925/'
    const res = await fetch('http://localhost:3000/graphql', {
        method: 'POST',
      
        headers: {
          "Content-Type": "application/json"
        },
      
        body: JSON.stringify({
          query: `{
            checkResearch(url: "${url}") {
            name
                authors {
                name
                id
                retractedArticles {
                    name
                    doi
                    pubmedId
                }
                }
                affiliations {
                name
                retractedArticles {
                    name
                    doi
                    pubmedId
                }
                }
                citations {
                name
                doi
                pubmedId
                retracted
                }
                fundings {
                name
                retractedArticles {
                    name
                    doi
                    pubmedId
                }
                }
                pubBase {
                name
                retractedArticles {
                    name
                    doi
                    pubmedId
                }
                }
                doi
                pubmedId
            }
            }`
        })
      })
    const record = await res.json()
    const checkedResearch = record.data.checkResearch

    console.log(checkedResearch)

    const name = checkedResearch.name

    const authors = checkedResearch.authors
    authors.filter((author) => author.retractedArticles.length > 0)
    authors.sort((a, b) => b.retractedArticles.length - a.retractedArticles.length)

    const affiliations = checkedResearch.affiliations
    affiliations.filter((affiliation) => affiliation.retractedArticles.length > 0)
    affiliations.sort((a, b) => b.retractedArticles.length - a.retractedArticles.length)

    const citations = checkedResearch.citations
    citations.filter((citation) => citation.retracted)

    const fundings = checkedResearch.fundings
    fundings.filter((funding) => funding.retractedArticles.length > 0)
    fundings.sort((a, b) => b.retractedArticles.length - a.retractedArticles.length)

    const pubBase = checkedResearch.pubBase
    const doi = checkedResearch.doi
    const pubmedId = checkedResearch.pubmedId


    document.getElementById("authors").innerHTML = checkedResearch.authors.map(
        author => `<li>${author.name}: ${author.retractedArticles.length}</li>`
    ).join('');
    document.getElementById("affiliations").innerHTML = checkedResearch.affiliations.map(
        affiliation => `<li>${affiliation.name}: ${affiliation.retractedArticles.length}</li>`
    ).join('');
    document.getElementById("citations").innerHTML = checkedResearch.citations.map(
        citation => `<li>${citation.name}</li>`
    ).join('');
    document.getElementById("fundings").innerHTML = checkedResearch.fundings.map(
        funding => `<li>${funding.name}: ${funding.retractedArticles.length}</li>`
    ).join('');
    document.getElementById("pubBase").innerHTML = `${pubBase.name}: ${pubBase.retractedArticles.length}`;
    document.getElementById("name").innerHTML = name
}
fetchData(); 