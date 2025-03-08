import { Module } from '@nestjs/common';
import { CheckerResolver } from './checker.resolver';
import { ResearchCheckerServiceClient } from './dto/research_checker.client';

@Module({
  providers: [CheckerResolver, ResearchCheckerServiceClient],
})
export class RecipesModule {}