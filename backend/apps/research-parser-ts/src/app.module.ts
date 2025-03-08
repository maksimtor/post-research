import { Module } from '@nestjs/common';
import { ResearchParserModule } from './research-parser-service/research-parser.module';

@Module({
  imports: [ResearchParserModule],
})
export class AppModule {}