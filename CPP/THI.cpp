#include <iostream>
#include <stddef.h>

int main(){

	const int maxIterations = 10;

	LPWSTR strInput;
		wscanf( L"%s", strInput );

	LPWSTR strResult = NULL;

	HANDLE hHeap = GetProcessHeap();

	int iSizeEstimated = NormalizeString( NormalizationD, strInput, -1, NULL, 0 );
	//									  Normalization Form, input, size(-1 as auto calc), set length.(stop point), set buffer size(0 as auto)
	for ( int i = 0; i < maxIterations; i++ ){
	
		if ( strResult )
			HeapFree( hHeap, 0, strResult );
			strResult = (LPWSTR)HeapAlloc( hHeap, 0, iSizeEstimated * sizeof (WCHAR) );
			iSizeEstimated = NormalizeString( NormalizationD, strInput, -1, strResult, iSizeEstimated );
    
		if ( iSizeEstimated > 0 )
			break; // success 
	
	}    

	TRACE(L"%x", strInput[0]);
	TRACE(L"%x", strResult[0]);
	TRACE(L"%x", strResult[1]);

}