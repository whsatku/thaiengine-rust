#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define c_time long int

typedef struct fileid{
	uint16_t magicKey;
	uint16_t type;
	uint16_t entrySize;
	uint16_t _filler;
	uint32_t numEntry;
	c_time timestamp;
} FILEID;

typedef struct data_record_st{
	uint32_t id;
	uint16_t lang:2;
	uint16_t length:10;
	uint8_t tailSpace:1;
	uint8_t isUnused:1;
	uint8_t numeric:1;
	uint8_t _filler:1;
	uint32_t mapFilePos;
	c_time timestamp;
	char szText[1023];
} DATA_RECORD;

void syllable_read_metadata(FILE* fp, FILEID* fileid){
	fread(fileid, sizeof(FILEID), 1, fp);
}

void syllable_skip_to_data(FILE* fp){
	fseek(fp, 256, SEEK_SET);
}

void syllable_read_record(FILE* fp, DATA_RECORD* record){
	fread(record, sizeof(DATA_RECORD)-(sizeof(char)*1024), 1, fp);

	if(record->tailSpace == 1){
		fread(&record->szText, sizeof(char), 1023, fp);
	}else{
		fread(&record->szText, sizeof(char), record->length + 1, fp);
	}
}

#ifdef STANDALONE
int main(int argc, char *argv[]){
	FILE* fp = fopen(argv[1], "rb");
	syllable_skip_to_data(fp);

	DATA_RECORD* record = (DATA_RECORD*) malloc(sizeof(DATA_RECORD));
	while(!feof(fp)){
		fread(record, sizeof(DATA_RECORD)-(sizeof(char)*1024), 1, fp);

		if(record->tailSpace == 1){
			fread(&record->szText, sizeof(char), 1023, fp);
		}else{
			fread(&record->szText, sizeof(char), record->length + 1, fp);
		}

		printf(
			"record %d lang %d length %d tailSpace %d unused %d numeric %d mapfilepos %d timestamp %ld\n",
			record->id, record->lang, record->length, record->tailSpace,
			record->isUnused, record->numeric, record->mapFilePos,
			record->timestamp
		);
		printf("%s\n=========\n", record->szText);
	}

	return 0;
}
#endif
