import { Controller, Inject, OnModuleInit } from '@nestjs/common';
import { ClientGrpc, GrpcMethod } from '@nestjs/microservices';
import { ParseRequest, ParseResponse } from './interfaces/research_parser';

import got from 'got';
import { JSDOM } from 'jsdom';

interface ResearchParserService {
  parse(data: ParseRequest): ParseResponse;
}

@Controller('research-parser')
export class ResearchParserController implements OnModuleInit {
  private researchParserService: ResearchParserService;

  constructor(@Inject('HERO_PACKAGE') private readonly client: ClientGrpc) {}

  onModuleInit() {
    this.researchParserService = this.client.getService<ResearchParserService>('ResearchParserService');
  }

  @GrpcMethod('ResearchParserService', 'Parse')
  async parse(data: ParseRequest): Promise<ParseResponse> {
    const authors = [];
    const affiliations = [];
    const fundings = [];
    const citations = [];

    const response = await got(data.url);
    const dom = new JSDOM(response.body);

    const headlineHtml = dom.window.document.getElementsByClassName('heading-title')[0].textContent;
    const authorsHtml = dom.window.document.getElementsByClassName('inline-authors');
    const pubBase = dom.window.document.getElementById('full-view-journal-trigger').title;
    const doi = dom.window.document.getElementsByClassName('id-link')[0].textContent;
    const affiliationsHtml = dom.window.document.getElementsByClassName('affiliations')[0].getElementsByTagName('li');
    const citationsHtml = dom.window.document.getElementsByClassName('refs-list')[0].getElementsByClassName('skip-numbering');

    // from here
    try {
      for (const citationHtml of citationsHtml) {
        let doi = '';
        let pubmedId = '';
  
        for (const lin of citationHtml.getElementsByTagName('a')) {
          if (lin.textContent.trim() === 'PubMed') {
            pubmedId = lin.getAttribute('data-ga-action');
          } else {
            doi = lin.getAttribute('data-ga-action');
          }
        }
  
        citations.push({
          name: citationHtml.textContent.trim().split('\n')[0],
          doi: doi,
          pubmedId: pubmedId,
        });
      }
    } catch (e) {
      console.log(e);
    }

    try {
      for (const affHtml of affiliationsHtml) {
        affiliations.push({
          name: affHtml.textContent.split(' ').slice(1).join(' '),
        });
      } 
    } catch (e) {
      console.log(e);
    }

    try {
      const grantsHtml = dom.window.document.getElementsByClassName('grants-list')[0].getElementsByTagName('a');
      for (const grantHtml of grantsHtml) {
        const f = grantHtml.title.split('/');
        fundings.push({ name: f[1] });
      }
    } catch (e) {
      console.log(e);
    }

    try {
      if (authorsHtml.length > 0) {
        const parsedAuthors = authorsHtml[0];
        const names = parsedAuthors.getElementsByClassName('full-name');
  
        for (const name of names) {
          authors.push({ name: name.textContent, id: 'undefined' });
        }
      }
    } catch (e) {
      console.log(e);
    }

    // to here
    return {
      pubBase: { name: pubBase },
      name: headlineHtml.trim(),
      authors: authors,
      affiliations: affiliations,
      citations: citations,
      fundings: fundings,
      doi: doi.trim(),
    };
  }
}
