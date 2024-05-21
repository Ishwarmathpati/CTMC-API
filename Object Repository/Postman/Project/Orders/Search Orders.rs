<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Orders</name>
   <tag></tag>
   <elementGuidId>b7a28013-8d76-4bd2-b09e-6a596c8ef074</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot; : {\n    \&quot;and\&quot; : [ {\n      \&quot;fullText\&quot; : {\n        \&quot;field\&quot; : \&quot;customLineItems.name\&quot;,\n        \&quot;language\&quot; : \&quot;en\&quot;,\n        \&quot;value\&quot; : \&quot;banana\&quot;\n      }\n    }, {\n      \&quot;filter\&quot; : [ {\n        \&quot;exact\&quot; : {\n          \&quot;field\&quot; : \&quot;store.name\&quot;,\n          \&quot;language\&quot; : \&quot;en\&quot;,\n          \&quot;value\&quot; : \&quot;fruit_store\&quot;\n        }\n      } ]\n    } ]\n  },\n  \&quot;sort\&quot; : [ {\n    \&quot;field\&quot; : \&quot;customLineItems.name\&quot;,\n    \&quot;language\&quot; : \&quot;en\&quot;,\n    \&quot;order\&quot; : \&quot;desc\&quot;\n  } ],\n  \&quot;limit\&quot; : 50,\n  \&quot;offset\&quot; : 0\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>01c0abaf-27ef-458a-b947-9acb6f386952</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>563f658d-6759-414f-858f-41d8698b7971</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/orders/search</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>6fa1bb09-3c51-4dd2-b6a1-b581d59e36e9</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>f689b803-a871-4463-8498-655cf6fb18e4</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
