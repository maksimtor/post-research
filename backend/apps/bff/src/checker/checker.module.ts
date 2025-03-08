import { Module } from '@nestjs/common';
// import { DateScalar } from '../common/scalars/date.scalar';
import { CheckerResolver } from './checker.resolver';
import { ResearchCheckerServiceClient } from './dto/research_checker.client';

@Module({
  providers: [CheckerResolver, ResearchCheckerServiceClient],
})
export class RecipesModule {}