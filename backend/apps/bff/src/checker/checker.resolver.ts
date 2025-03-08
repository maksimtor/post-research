import { NotFoundException } from '@nestjs/common';
import {
  Args,
  Query,
  Resolver,
} from '@nestjs/graphql';
import { CheckedResearch } from './models/checked-research.model';
import { ResearchCheckerServiceClient } from './dto/research_checker.client';
import { GrpcTransport } from '@protobuf-ts/grpc-transport';
import { ChannelCredentials } from '@grpc/grpc-js';

@Resolver(() => CheckedResearch)
export class CheckerResolver {
  constructor(private readonly recipesService: ResearchCheckerServiceClient) {}

  @Query(() => CheckedResearch)
  async checkResearch(@Args('url') url: string): Promise<CheckedResearch> {
    const transport = new GrpcTransport({
      host: 'localhost:3001',
      channelCredentials: ChannelCredentials.createInsecure(),
    });

    const client = new ResearchCheckerServiceClient(transport);
    const recipe = await client.check({ url: url });

    if (!recipe) {
      throw new NotFoundException(url);
    }

    return {
      name: recipe.response.name,
      authors: recipe.response.authors,
      affiliations: recipe.response.affiliations,
      citations: recipe.response.citations,
      fundings: recipe.response.fundings,
      pubBase: recipe.response.pubBase,
      doi: recipe.response.doi,
      pubmedId: recipe.response.pubmedId,
    };
  }
}
